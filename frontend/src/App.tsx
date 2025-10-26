/**
 * Main App component for Chatapp frontend.
 */

import React, { useEffect, useState, useCallback } from 'react';
import { WebSocketConnection } from './connection';
import { renderElement, RenderContext } from './renderer';
import { StateManager, createStateManager } from './state';
import { EventManager, createEventManager } from './events';
import {
  initializeProto,
  deserializeForwardMsg,
  extractElements,
  extractDeltas,
  getForwardMsgType,
} from './proto';

export function App() {
  const [state, setState] = useState({
    connected: false,
    elements: [] as any[],
    loading: true,
    error: null as string | null,
    sessionId: '',
  });

  const connectionRef = React.useRef<WebSocketConnection | null>(null);
  const stateManagerRef = React.useRef<StateManager | null>(null);
  const eventManagerRef = React.useRef<EventManager | null>(null);

  // Initialize proto and WebSocket connection
  useEffect(() => {
    const initializeApp = async () => {
      try {
        // Initialize proto definitions
        await initializeProto();

        // Create connection
        const connection = new WebSocketConnection({
          url: `ws://${window.location.hostname}:8000/ws`,
          reconnectInterval: 3000,
          maxReconnectAttempts: 10,
        });

        connectionRef.current = connection;

        // Create state manager
        const stateManager = createStateManager('temp-session-id');
        stateManagerRef.current = stateManager;

        // Create event manager
        const eventManager = createEventManager(connection, stateManager);
        eventManagerRef.current = eventManager;

        // Handle connection state changes
        connection.onStateChange((connected) => {
          stateManager.setConnected(connected);
          setState((prev) => ({ ...prev, connected, loading: !connected }));
        });

        // Handle incoming messages
        connection.onMessage((data) => {
          handleMessage(data, stateManager);
        });

        // Connect to server
        await connection.connect();
        stateManager.setLoading(false);
      } catch (error) {
        console.error('Failed to initialize app:', error);
        setState((prev) => ({
          ...prev,
          error: `Failed to initialize: ${error}`,
          loading: false,
        }));
      }
    };

    initializeApp();

    // Cleanup on unmount
    return () => {
      connectionRef.current?.disconnect();
      eventManagerRef.current?.destroy();
    };
  }, []);

  /**
   * Handle incoming proto messages from server
   */
  const handleMessage = useCallback(
    (data: ArrayBuffer, stateManager: StateManager) => {
      try {
        const bytes = new Uint8Array(data);
        console.log('Received message:', bytes.length, 'bytes');

        // Deserialize proto message
        const forwardMsg = deserializeForwardMsg(bytes);
        const msgType = getForwardMsgType(forwardMsg);

        console.log('Message type:', msgType);

        if (msgType === 'newSession') {
          // Extract session ID and initial elements
          const elements = extractElements(forwardMsg);
          stateManager.addElements(
            elements.map((el: any) => ({
              id: el.id,
              type: el.type?.oneofKind || 'unknown',
              props: el,
            }))
          );

          setState((prev) => ({
            ...prev,
            sessionId: forwardMsg.type?.newSession?.sessionId || '',
            elements: elements,
            error: null,
          }));
        } else if (msgType === 'delta') {
          // Apply delta updates
          const deltas = extractDeltas(forwardMsg);
          deltas.forEach((delta: any) => {
            stateManager.applyDelta(delta);
          });

          // Update UI with new elements
          const allElements = stateManager.getElements();
          setState((prev) => ({
            ...prev,
            elements: allElements,
            error: null,
          }));
        } else if (msgType === 'scriptFinished') {
          console.log('Script finished:', forwardMsg.type?.scriptFinished?.status);
        } else if (msgType === 'error') {
          const errorMsg = forwardMsg.type?.error?.message || 'Unknown error';
          stateManager.setError(errorMsg);
          setState((prev) => ({
            ...prev,
            error: errorMsg,
          }));
        }
      } catch (error) {
        console.error('Failed to parse message:', error);
        setState((prev) => ({
          ...prev,
          error: `Failed to parse server message: ${error}`,
        }));
      }
    },
    []
  );

  /**
   * Handle widget state changes
   */
  const handleWidgetChange = useCallback((key: string, value: unknown) => {
    if (!eventManagerRef.current) {
      console.error('Event manager not initialized');
      return;
    }
    eventManagerRef.current.handleWidgetChange('widget-' + key, key, value);
  }, []);

  /**
   * Handle button clicks
   */
  const handleButtonClick = useCallback((key: string) => {
    if (!eventManagerRef.current) {
      console.error('Event manager not initialized');
      return;
    }
    eventManagerRef.current.handleButtonClick('button-' + key, key);
  }, []);

  const renderContext: RenderContext = {
    onWidgetChange: handleWidgetChange,
    onButtonClick: handleButtonClick,
  };

  return (
    <div style={{ minHeight: '100vh', backgroundColor: '#f5f5f5' }}>
      {/* Header */}
      <header
        style={{
          backgroundColor: '#0066cc',
          color: 'white',
          padding: '1rem',
          boxShadow: '0 2px 4px rgba(0,0,0,0.1)',
        }}
      >
        <div style={{ maxWidth: '1200px', margin: '0 auto' }}>
          <h1 style={{ margin: 0 }}>Chatapp</h1>
          <p style={{ margin: '0.5rem 0 0 0', fontSize: '0.875rem', opacity: 0.9 }}>
            {state.connected ? '✓ Connected' : '○ Connecting...'}
          </p>
        </div>
      </header>

      {/* Main Content */}
      <main style={{ maxWidth: '1200px', margin: '0 auto', padding: '2rem 1rem' }}>
        {/* Loading State */}
        {state.loading && (
          <div
            style={{
              textAlign: 'center',
              padding: '2rem',
              color: '#666',
            }}
          >
            <p>Connecting to server...</p>
          </div>
        )}

        {/* Error State */}
        {state.error && (
          <div
            style={{
              padding: '1rem',
              backgroundColor: '#f8d7da',
              color: '#721c24',
              borderRadius: '4px',
              marginBottom: '1rem',
              border: '1px solid #f5c6cb',
            }}
          >
            Error: {state.error}
          </div>
        )}

        {/* Elements */}
        {!state.loading && state.elements.length === 0 && state.connected && (
          <div
            style={{
              textAlign: 'center',
              padding: '2rem',
              color: '#999',
            }}
          >
            <p>Waiting for app content...</p>
          </div>
        )}

        {/* Render elements */}
        <div style={{ backgroundColor: 'white', padding: '2rem', borderRadius: '4px' }}>
          {state.elements.map((element) => {
            const rendered = renderElement(element, renderContext);
            return (
              <div key={rendered.id}>
                {rendered.component}
              </div>
            );
          })}
        </div>
      </main>

      {/* Footer */}
      <footer
        style={{
          backgroundColor: '#f0f0f0',
          padding: '1rem',
          textAlign: 'center',
          color: '#666',
          fontSize: '0.875rem',
          marginTop: '2rem',
          borderTop: '1px solid #ddd',
        }}
      >
        <p>Chatapp © 2025</p>
      </footer>
    </div>
  );
}
