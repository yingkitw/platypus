/**
 * Simple client-side router for multi-page app support.
 */

import React from 'react';

export interface Route {
  path: string;
  name: string;
  component: any;
  meta?: {
    title?: string;
    description?: string;
    requiresAuth?: boolean;
  };
}

export interface RouteParams {
  [key: string]: string;
}

export interface NavigationOptions {
  replace?: boolean;
  state?: any;
}

/**
 * Simple router implementation
 */
export class Router {
  private routes: Route[] = [];
  private currentPath: string = '/';
  private listeners: Set<(path: string) => void> = new Set();
  private history: string[] = ['/'];
  private historyIndex: number = 0;

  /**
   * Register routes
   */
  registerRoutes(routes: Route[]): void {
    this.routes = routes;
  }

  /**
   * Add a route
   */
  addRoute(route: Route): void {
    this.routes.push(route);
  }

  /**
   * Navigate to a path
   */
  navigate(path: string, options: NavigationOptions = {}): void {
    // Normalize path
    const normalizedPath = path.startsWith('/') ? path : `/${path}`;

    // Check if route exists
    if (!this.findRoute(normalizedPath)) {
      console.warn(`Route not found: ${normalizedPath}`);
      return;
    }

    if (options.replace) {
      this.history[this.historyIndex] = normalizedPath;
    } else {
      // Remove any forward history
      this.history = this.history.slice(0, this.historyIndex + 1);
      this.history.push(normalizedPath);
      this.historyIndex++;
    }

    this.currentPath = normalizedPath;
    this.notifyListeners();

    // Update browser URL
    if (typeof window !== 'undefined') {
      window.history.pushState(options.state || {}, '', normalizedPath);
    }
  }

  /**
   * Go back in history
   */
  back(): void {
    if (this.historyIndex > 0) {
      this.historyIndex--;
      this.currentPath = this.history[this.historyIndex];
      this.notifyListeners();
    }
  }

  /**
   * Go forward in history
   */
  forward(): void {
    if (this.historyIndex < this.history.length - 1) {
      this.historyIndex++;
      this.currentPath = this.history[this.historyIndex];
      this.notifyListeners();
    }
  }

  /**
   * Get current path
   */
  getCurrentPath(): string {
    return this.currentPath;
  }

  /**
   * Find route by path
   */
  findRoute(path: string): Route | null {
    // Exact match
    const exactMatch = this.routes.find((r) => r.path === path);
    if (exactMatch) return exactMatch;

    // Pattern match (e.g., /users/:id)
    for (const route of this.routes) {
      const pattern = this.pathToRegex(route.path);
      if (pattern.test(path)) {
        return route;
      }
    }

    return null;
  }

  /**
   * Get route params
   */
  getParams(path: string): RouteParams {
    const route = this.findRoute(path);
    if (!route) return {};

    const pattern = this.pathToRegex(route.path);
    const match = path.match(pattern);
    if (!match) return {};

    const params: RouteParams = {};
    const paramNames = this.extractParamNames(route.path);

    paramNames.forEach((name, index) => {
      params[name] = match[index + 1];
    });

    return params;
  }

  /**
   * Subscribe to route changes
   */
  subscribe(listener: (path: string) => void): () => void {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  }

  /**
   * Notify listeners of route change
   */
  private notifyListeners(): void {
    this.listeners.forEach((listener) => listener(this.currentPath));
  }

  /**
   * Convert path pattern to regex
   */
  private pathToRegex(path: string): RegExp {
    const pattern = path
      .replace(/\//g, '\\/')
      .replace(/:(\w+)/g, '([^/]+)');
    return new RegExp(`^${pattern}$`);
  }

  /**
   * Extract parameter names from path
   */
  private extractParamNames(path: string): string[] {
    const matches = path.match(/:(\w+)/g);
    return matches ? matches.map((m) => m.substring(1)) : [];
  }
}

/**
 * Create a router instance
 */
export function createRouter(): Router {
  return new Router();
}

/**
 * Link component for navigation
 */
export interface LinkProps {
  to: string;
  children: React.ReactNode;
  className?: string;
  style?: React.CSSProperties;
  onClick?: (e: React.MouseEvent) => void;
}

/**
 * useRouter hook
 */
export function useRouter(router: Router): {
  currentPath: string;
  navigate: (path: string, options?: NavigationOptions) => void;
  back: () => void;
  forward: () => void;
  getParams: (path?: string) => RouteParams;
} {
  const [currentPath, setCurrentPath] = React.useState(router.getCurrentPath());

  React.useEffect(() => {
    const unsubscribe = router.subscribe((path) => {
      setCurrentPath(path);
    });

    return unsubscribe;
  }, [router]);

  return {
    currentPath,
    navigate: (path, options) => router.navigate(path, options),
    back: () => router.back(),
    forward: () => router.forward(),
    getParams: (path) => router.getParams(path || currentPath),
  };
}
