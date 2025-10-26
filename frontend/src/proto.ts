/**
 * Protocol Buffer message handling for Chatapp.
 * Provides serialization/deserialization of proto messages.
 */

import * as protobufjs from 'protobufjs';

let root: protobufjs.Root | null = null;

/**
 * Initialize proto definitions
 */
export async function initializeProto(): Promise<void> {
  if (root) return;

  // Load proto definitions
  root = await protobufjs.load('/proto/element.proto');
  await protobufjs.load('/proto/forward_msg.proto');
  await protobufjs.load('/proto/back_msg.proto');
}

/**
 * Get a proto message type
 */
function getMessageType(name: string): protobufjs.Type {
  if (!root) {
    throw new Error('Proto not initialized. Call initializeProto() first.');
  }
  const type = root.lookupType(name);
  if (!type) {
    throw new Error(`Proto type not found: ${name}`);
  }
  return type;
}

/**
 * Deserialize ForwardMsg from bytes
 */
export function deserializeForwardMsg(bytes: Uint8Array): any {
  const ForwardMsg = getMessageType('platypus.ForwardMsg');
  return ForwardMsg.decode(bytes);
}

/**
 * Serialize BackMsg to bytes
 */
export function serializeBackMsg(msg: any): Uint8Array {
  const BackMsg = getMessageType('platypus.BackMsg');
  const errMsg = BackMsg.verify(msg);
  if (errMsg) {
    throw new Error(`BackMsg verification failed: ${errMsg}`);
  }
  return BackMsg.encode(msg).finish();
}

/**
 * Create a WidgetStateChangeMsg
 */
export function createWidgetStateChangeMsg(
  sessionId: string,
  widgetKey: string,
  value: unknown
): any {
  return {
    sessionId,
    type: {
      oneofKind: 'widgetStateChange',
      widgetStateChange: {
        widgetKey,
        value: JSON.stringify(value),
      },
    },
  };
}

/**
 * Create a RerunScriptMsg
 */
export function createRerunScriptMsg(sessionId: string): any {
  return {
    sessionId,
    type: {
      oneofKind: 'rerunScript',
      rerunScript: {},
    },
  };
}

/**
 * Create a UserInteractionMsg
 */
export function createUserInteractionMsg(
  sessionId: string,
  elementId: string,
  interactionType: string,
  data?: unknown
): any {
  return {
    sessionId,
    type: {
      oneofKind: 'userInteraction',
      userInteraction: {
        elementId,
        interactionType,
        data: data ? JSON.stringify(data) : '',
      },
    },
  };
}

/**
 * Extract elements from ForwardMsg
 */
export function extractElements(forwardMsg: any): any[] {
  const elements: any[] = [];

  if (forwardMsg.type?.oneofKind === 'newSession') {
    elements.push(...(forwardMsg.type.newSession.elements || []));
  } else if (forwardMsg.type?.oneofKind === 'delta') {
    const deltas = forwardMsg.type.delta.deltas || [];
    for (const delta of deltas) {
      if (delta.type?.oneofKind === 'addElement') {
        elements.push(delta.type.addElement.element);
      }
    }
  }

  return elements;
}

/**
 * Extract delta updates from ForwardMsg
 */
export function extractDeltas(forwardMsg: any): any[] {
  if (forwardMsg.type?.oneofKind === 'delta') {
    return forwardMsg.type.delta.deltas || [];
  }
  return [];
}

/**
 * Get message type from ForwardMsg
 */
export function getForwardMsgType(forwardMsg: any): string {
  return forwardMsg.type?.oneofKind || 'unknown';
}

/**
 * Parse widget value from string
 */
export function parseWidgetValue(value: string): unknown {
  try {
    return JSON.parse(value);
  } catch {
    return value;
  }
}
