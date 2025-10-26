/**
 * Element state management for Chatapp frontend.
 * Manages UI element state, widget values, and state updates.
 */

export interface ElementState {
  id: string;
  type: string;
  props: Record<string, unknown>;
  children?: string[];
  value?: unknown;
}

export interface WidgetState {
  [key: string]: unknown;
}

export interface AppState {
  sessionId: string;
  elements: Map<string, ElementState>;
  widgetValues: WidgetState;
  connected: boolean;
  loading: boolean;
  error: string | null;
}

/**
 * State manager for the application
 */
export class StateManager {
  private state: AppState;
  private listeners: Set<(state: AppState) => void> = new Set();

  constructor(sessionId: string) {
    this.state = {
      sessionId,
      elements: new Map(),
      widgetValues: {},
      connected: false,
      loading: true,
      error: null,
    };
  }

  /**
   * Get current state
   */
  getState(): AppState {
    return this.state;
  }

  /**
   * Subscribe to state changes
   */
  subscribe(listener: (state: AppState) => void): () => void {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  }

  /**
   * Notify all listeners of state change
   */
  private notifyListeners(): void {
    this.listeners.forEach((listener) => listener(this.state));
  }

  /**
   * Add or update an element
   */
  addElement(element: ElementState): void {
    this.state.elements.set(element.id, element);
    this.notifyListeners();
  }

  /**
   * Add multiple elements
   */
  addElements(elements: ElementState[]): void {
    elements.forEach((el) => this.state.elements.set(el.id, el));
    this.notifyListeners();
  }

  /**
   * Remove an element
   */
  removeElement(id: string): void {
    this.state.elements.delete(id);
    this.notifyListeners();
  }

  /**
   * Clear all elements
   */
  clearElements(): void {
    this.state.elements.clear();
    this.notifyListeners();
  }

  /**
   * Get an element by ID
   */
  getElement(id: string): ElementState | undefined {
    return this.state.elements.get(id);
  }

  /**
   * Get all elements
   */
  getElements(): ElementState[] {
    return Array.from(this.state.elements.values());
  }

  /**
   * Update widget value
   */
  setWidgetValue(key: string, value: unknown): void {
    this.state.widgetValues[key] = value;
    this.notifyListeners();
  }

  /**
   * Get widget value
   */
  getWidgetValue(key: string): unknown {
    return this.state.widgetValues[key];
  }

  /**
   * Update multiple widget values
   */
  setWidgetValues(values: WidgetState): void {
    this.state.widgetValues = { ...this.state.widgetValues, ...values };
    this.notifyListeners();
  }

  /**
   * Clear widget values
   */
  clearWidgetValues(): void {
    this.state.widgetValues = {};
    this.notifyListeners();
  }

  /**
   * Set connection state
   */
  setConnected(connected: boolean): void {
    this.state.connected = connected;
    this.notifyListeners();
  }

  /**
   * Set loading state
   */
  setLoading(loading: boolean): void {
    this.state.loading = loading;
    this.notifyListeners();
  }

  /**
   * Set error message
   */
  setError(error: string | null): void {
    this.state.error = error;
    this.notifyListeners();
  }

  /**
   * Apply delta updates to elements
   */
  applyDelta(delta: any): void {
    if (delta.type?.oneofKind === 'addElement') {
      const { element, parentId } = delta.type.addElement;
      if (element) {
        this.addElement({
          id: element.id,
          type: this.getElementType(element),
          props: this.extractProps(element),
          children: parentId ? [parentId] : undefined,
        });
      }
    } else if (delta.type?.oneofKind === 'updateElement') {
      const { element } = delta.type.updateElement;
      if (element) {
        const existing = this.getElement(element.id);
        if (existing) {
          this.addElement({
            ...existing,
            props: this.extractProps(element),
          });
        }
      }
    } else if (delta.type?.oneofKind === 'removeElement') {
      const { elementId } = delta.type.removeElement;
      this.removeElement(elementId);
    } else if (delta.type?.oneofKind === 'clearContainer') {
      const { containerId } = delta.type.clearContainer;
      // Remove all children of container
      const toRemove: string[] = [];
      this.state.elements.forEach((el, id) => {
        if (el.children?.includes(containerId)) {
          toRemove.push(id);
        }
      });
      toRemove.forEach((id) => this.removeElement(id));
    }
  }

  /**
   * Get element type from proto element
   */
  private getElementType(element: any): string {
    const type = element.type;
    if (!type) return 'unknown';

    if (type.oneofKind === 'text') return 'text';
    if (type.oneofKind === 'markdown') return 'markdown';
    if (type.oneofKind === 'heading') return 'heading';
    if (type.oneofKind === 'button') return 'button';
    if (type.oneofKind === 'textInput') return 'textInput';
    if (type.oneofKind === 'slider') return 'slider';
    if (type.oneofKind === 'checkbox') return 'checkbox';
    if (type.oneofKind === 'selectbox') return 'selectbox';
    if (type.oneofKind === 'success') return 'success';
    if (type.oneofKind === 'error') return 'error';
    if (type.oneofKind === 'warning') return 'warning';
    if (type.oneofKind === 'info') return 'info';
    if (type.oneofKind === 'progress') return 'progress';
    if (type.oneofKind === 'divider') return 'divider';

    return type.oneofKind || 'unknown';
  }

  /**
   * Extract props from proto element
   */
  private extractProps(element: any): Record<string, unknown> {
    const type = element.type;
    if (!type) return {};

    const props: Record<string, unknown> = {};

    if (type.text) {
      props.value = type.text.value;
    } else if (type.markdown) {
      props.value = type.markdown.value;
    } else if (type.heading) {
      props.value = type.heading.value;
      props.level = type.heading.level;
    } else if (type.button) {
      props.label = type.button.label;
      props.key = type.button.key;
    } else if (type.textInput) {
      props.label = type.textInput.label;
      props.value = type.textInput.value;
      props.key = type.textInput.key;
    } else if (type.slider) {
      props.label = type.slider.label;
      props.value = type.slider.value;
      props.min = type.slider.min;
      props.max = type.slider.max;
      props.key = type.slider.key;
    } else if (type.checkbox) {
      props.label = type.checkbox.label;
      props.value = type.checkbox.value;
      props.key = type.checkbox.key;
    } else if (type.selectbox) {
      props.label = type.selectbox.label;
      props.options = type.selectbox.options;
      props.value = type.selectbox.value;
      props.key = type.selectbox.key;
    } else if (type.success || type.error || type.warning || type.info) {
      const msgType = type.success || type.error || type.warning || type.info;
      props.message = msgType.message;
    } else if (type.progress) {
      props.value = type.progress.value;
    }

    return props;
  }
}

/**
 * Create a state manager instance
 */
export function createStateManager(sessionId: string): StateManager {
  return new StateManager(sessionId);
}
