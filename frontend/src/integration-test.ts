/**
 * Integration test utilities for proto message communication.
 * Tests the end-to-end flow between frontend and backend.
 */

import { WebSocketConnection } from './connection';
import { StateManager, createStateManager } from './state';
import { EventManager, createEventManager } from './events';
import {
  initializeProto,
  deserializeForwardMsg,
  serializeBackMsg,
  createWidgetStateChangeMsg,
} from './proto';

export interface IntegrationTestResult {
  name: string;
  passed: boolean;
  duration: number;
  error?: string;
}

export class IntegrationTester {
  private results: IntegrationTestResult[] = [];
  private connection: WebSocketConnection | null = null;
  private stateManager: StateManager | null = null;
  private eventManager: EventManager | null = null;

  /**
   * Run all integration tests
   */
  async runAllTests(): Promise<IntegrationTestResult[]> {
    console.log('🧪 Starting integration tests...\n');

    await this.testProtoInitialization();
    await this.testWebSocketConnection();
    await this.testStateManagement();
    await this.testEventPropagation();
    await this.testMessageSerialization();

    this.printResults();
    return this.results;
  }

  /**
   * Test proto initialization
   */
  private async testProtoInitialization(): Promise<void> {
    const startTime = Date.now();
    const testName = 'Proto Initialization';

    try {
      await initializeProto();
      this.addResult(testName, true, Date.now() - startTime);
      console.log('✅ Proto initialized successfully');
    } catch (error) {
      this.addResult(testName, false, Date.now() - startTime, String(error));
      console.log('❌ Proto initialization failed:', error);
    }
  }

  /**
   * Test WebSocket connection
   */
  private async testWebSocketConnection(): Promise<void> {
    const startTime = Date.now();
    const testName = 'WebSocket Connection';

    try {
      this.connection = new WebSocketConnection({
        url: 'ws://localhost:8000/ws',
        reconnectInterval: 1000,
        maxReconnectAttempts: 3,
      });

      // Try to connect with timeout
      const connectPromise = this.connection.connect();
      const timeoutPromise = new Promise((_, reject) =>
        setTimeout(() => reject(new Error('Connection timeout')), 5000)
      );

      await Promise.race([connectPromise, timeoutPromise]);
      this.addResult(testName, true, Date.now() - startTime);
      console.log('✅ WebSocket connected successfully');
    } catch (error) {
      this.addResult(testName, false, Date.now() - startTime, String(error));
      console.log('⚠️  WebSocket connection failed (expected if server not running):', error);
    }
  }

  /**
   * Test state management
   */
  private async testStateManagement(): Promise<void> {
    const startTime = Date.now();
    const testName = 'State Management';

    try {
      this.stateManager = createStateManager('test-session');

      // Test adding element
      this.stateManager.addElement({
        id: 'elem-1',
        type: 'text',
        props: { value: 'Hello' },
      });

      // Test retrieving element
      const element = this.stateManager.getElement('elem-1');
      if (!element) throw new Error('Element not found');

      // Test widget value
      this.stateManager.setWidgetValue('widget-1', 'test-value');
      const value = this.stateManager.getWidgetValue('widget-1');
      if (value !== 'test-value') throw new Error('Widget value mismatch');

      this.addResult(testName, true, Date.now() - startTime);
      console.log('✅ State management working correctly');
    } catch (error) {
      this.addResult(testName, false, Date.now() - startTime, String(error));
      console.log('❌ State management test failed:', error);
    }
  }

  /**
   * Test event propagation
   */
  private async testEventPropagation(): Promise<void> {
    const startTime = Date.now();
    const testName = 'Event Propagation';

    try {
      if (!this.connection || !this.stateManager) {
        throw new Error('Connection or state manager not initialized');
      }

      this.eventManager = createEventManager(this.connection, this.stateManager);

      // Test event listener
      let eventFired = false;
      this.eventManager.on('change', () => {
        eventFired = true;
      });

      // Simulate widget change (won't send to server if not connected)
      this.eventManager.handleWidgetChange('elem-1', 'widget-1', 'new-value');

      // Check if event was fired
      if (!eventFired) throw new Error('Event not fired');

      this.addResult(testName, true, Date.now() - startTime);
      console.log('✅ Event propagation working correctly');
    } catch (error) {
      this.addResult(testName, false, Date.now() - startTime, String(error));
      console.log('❌ Event propagation test failed:', error);
    }
  }

  /**
   * Test message serialization
   */
  private async testMessageSerialization(): Promise<void> {
    const startTime = Date.now();
    const testName = 'Message Serialization';

    try {
      // Test creating and serializing a message
      const msg = createWidgetStateChangeMsg('session-1', 'widget-1', 'test-value');
      const bytes = serializeBackMsg(msg);

      if (!bytes || bytes.length === 0) {
        throw new Error('Serialization produced empty bytes');
      }

      this.addResult(testName, true, Date.now() - startTime);
      console.log('✅ Message serialization working correctly');
    } catch (error) {
      this.addResult(testName, false, Date.now() - startTime, String(error));
      console.log('❌ Message serialization test failed:', error);
    }
  }

  /**
   * Add test result
   */
  private addResult(
    name: string,
    passed: boolean,
    duration: number,
    error?: string
  ): void {
    this.results.push({ name, passed, duration, error });
  }

  /**
   * Print test results
   */
  private printResults(): void {
    console.log('\n📊 Test Results:');
    console.log('================\n');

    let passedCount = 0;
    let totalDuration = 0;

    this.results.forEach((result) => {
      const status = result.passed ? '✅' : '❌';
      console.log(`${status} ${result.name}: ${result.duration}ms`);
      if (result.error) {
        console.log(`   Error: ${result.error}`);
      }
      if (result.passed) passedCount++;
      totalDuration += result.duration;
    });

    console.log('\n================');
    console.log(`Total: ${passedCount}/${this.results.length} passed`);
    console.log(`Duration: ${totalDuration}ms\n`);
  }

  /**
   * Cleanup
   */
  cleanup(): void {
    if (this.connection) {
      this.connection.disconnect();
    }
    if (this.eventManager) {
      this.eventManager.destroy();
    }
  }
}

/**
 * Run integration tests
 */
export async function runIntegrationTests(): Promise<void> {
  const tester = new IntegrationTester();
  try {
    await tester.runAllTests();
  } finally {
    tester.cleanup();
  }
}
