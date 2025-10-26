/**
 * Widget event handling and propagation system.
 * Manages user interactions and sends them to the server.
 */

import { WebSocketConnection } from './connection';
import { StateManager } from './state';
import {
  createWidgetStateChangeMsg,
  createUserInteractionMsg,
  serializeBackMsg,
} from './proto';

export type EventHandler = (event: WidgetEvent) => void;

export interface WidgetEvent {
  type: 'change' | 'click' | 'submit' | 'interaction';
  elementId: string;
  widgetKey?: string;
  value?: unknown;
  data?: Record<string, unknown>;
  timestamp: number;
}

/**
 * Event manager for handling widget interactions
 */
export class EventManager {
  private connection: WebSocketConnection;
  private state: StateManager;
  private handlers: Map<string, Set<EventHandler>> = new Map();
  private debounceTimers: Map<string, NodeJS.Timeout> = new Map();
  private debounceDelay = 300; // ms

  constructor(connection: WebSocketConnection, state: StateManager) {
    this.connection = connection;
    this.state = state;
  }

  /**
   * Register an event handler
   */
  on(eventType: string, handler: EventHandler): () => void {
    if (!this.handlers.has(eventType)) {
      this.handlers.set(eventType, new Set());
    }
    this.handlers.get(eventType)!.add(handler);

    return () => {
      const handlers = this.handlers.get(eventType);
      if (handlers) {
        handlers.delete(handler);
      }
    };
  }

  /**
   * Emit an event
   */
  private emit(event: WidgetEvent): void {
    const handlers = this.handlers.get(event.type);
    if (handlers) {
      handlers.forEach((handler) => handler(event));
    }
  }

  /**
   * Handle widget value change
   */
  handleWidgetChange(
    elementId: string,
    widgetKey: string,
    value: unknown
  ): void {
    // Update local state immediately for responsive UI
    this.state.setWidgetValue(widgetKey, value);

    // Emit event
    const event: WidgetEvent = {
      type: 'change',
      elementId,
      widgetKey,
      value,
      timestamp: Date.now(),
    };
    this.emit(event);

    // Debounce sending to server
    this.debounceServerUpdate(elementId, widgetKey, value);
  }

  /**
   * Handle button click
   */
  handleButtonClick(elementId: string, widgetKey?: string): void {
    // Emit event
    const event: WidgetEvent = {
      type: 'click',
      elementId,
      widgetKey,
      timestamp: Date.now(),
    };
    this.emit(event);

    // Send to server immediately
    this.sendButtonClickToServer(elementId, widgetKey);
  }

  /**
   * Handle form submission
   */
  handleFormSubmit(elementId: string, data: Record<string, unknown>): void {
    // Emit event
    const event: WidgetEvent = {
      type: 'submit',
      elementId,
      data,
      timestamp: Date.now(),
    };
    this.emit(event);

    // Send to server
    this.sendFormSubmitToServer(elementId, data);
  }

  /**
   * Handle generic user interaction
   */
  handleInteraction(
    elementId: string,
    interactionType: string,
    data?: Record<string, unknown>
  ): void {
    // Emit event
    const event: WidgetEvent = {
      type: 'interaction',
      elementId,
      data,
      timestamp: Date.now(),
    };
    this.emit(event);

    // Send to server
    this.sendInteractionToServer(elementId, interactionType, data);
  }

  /**
   * Debounce server update for widget changes
   */
  private debounceServerUpdate(
    elementId: string,
    widgetKey: string,
    value: unknown
  ): void {
    const key = `${elementId}:${widgetKey}`;

    // Clear existing timer
    const existingTimer = this.debounceTimers.get(key);
    if (existingTimer) {
      clearTimeout(existingTimer);
    }

    // Set new timer
    const timer = setTimeout(() => {
      this.sendWidgetChangeToServer(widgetKey, value);
      this.debounceTimers.delete(key);
    }, this.debounceDelay);

    this.debounceTimers.set(key, timer);
  }

  /**
   * Send widget change to server
   */
  private sendWidgetChangeToServer(widgetKey: string, value: unknown): void {
    if (!this.connection.isConnected()) {
      console.error('Not connected to server');
      return;
    }

    try {
      const sessionId = this.state.getState().sessionId;
      const msg = createWidgetStateChangeMsg(sessionId, widgetKey, value);
      const bytes = serializeBackMsg(msg);
      this.connection.send(bytes.buffer as ArrayBuffer);

      console.log('Sent widget change:', widgetKey, value);
    } catch (error) {
      console.error('Failed to send widget change:', error);
      this.state.setError(`Failed to send widget change: ${error}`);
    }
  }

  /**
   * Send button click to server
   */
  private sendButtonClickToServer(elementId: string, widgetKey?: string): void {
    if (!this.connection.isConnected()) {
      console.error('Not connected to server');
      return;
    }

    try {
      const sessionId = this.state.getState().sessionId;
      const msg = createUserInteractionMsg(
        sessionId,
        elementId,
        'click',
        widgetKey ? { key: widgetKey } : undefined
      );
      const bytes = serializeBackMsg(msg);
      this.connection.send(bytes.buffer as ArrayBuffer);

      console.log('Sent button click:', elementId);
    } catch (error) {
      console.error('Failed to send button click:', error);
      this.state.setError(`Failed to send button click: ${error}`);
    }
  }

  /**
   * Send form submission to server
   */
  private sendFormSubmitToServer(
    elementId: string,
    data: Record<string, unknown>
  ): void {
    if (!this.connection.isConnected()) {
      console.error('Not connected to server');
      return;
    }

    try {
      const sessionId = this.state.getState().sessionId;
      const msg = createUserInteractionMsg(
        sessionId,
        elementId,
        'submit',
        data
      );
      const bytes = serializeBackMsg(msg);
      this.connection.send(bytes.buffer as ArrayBuffer);

      console.log('Sent form submission:', elementId);
    } catch (error) {
      console.error('Failed to send form submission:', error);
      this.state.setError(`Failed to send form submission: ${error}`);
    }
  }

  /**
   * Send generic interaction to server
   */
  private sendInteractionToServer(
    elementId: string,
    interactionType: string,
    data?: Record<string, unknown>
  ): void {
    if (!this.connection.isConnected()) {
      console.error('Not connected to server');
      return;
    }

    try {
      const sessionId = this.state.getState().sessionId;
      const msg = createUserInteractionMsg(
        sessionId,
        elementId,
        interactionType,
        data
      );
      const bytes = serializeBackMsg(msg);
      this.connection.send(bytes.buffer as ArrayBuffer);

      console.log('Sent interaction:', elementId, interactionType);
    } catch (error) {
      console.error('Failed to send interaction:', error);
      this.state.setError(`Failed to send interaction: ${error}`);
    }
  }

  /**
   * Set debounce delay
   */
  setDebounceDelay(delay: number): void {
    this.debounceDelay = delay;
  }

  /**
   * Clear all debounce timers
   */
  clearDebounceTimers(): void {
    this.debounceTimers.forEach((timer) => clearTimeout(timer));
    this.debounceTimers.clear();
  }

  /**
   * Cleanup
   */
  destroy(): void {
    this.clearDebounceTimers();
    this.handlers.clear();
  }
}

/**
 * Create an event manager instance
 */
export function createEventManager(
  connection: WebSocketConnection,
  state: StateManager
): EventManager {
  return new EventManager(connection, state);
}
