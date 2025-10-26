/**
 * Performance optimization utilities for Chatapp frontend.
 * Includes memoization, lazy loading, and rendering optimizations.
 */

/**
 * Memoize function results
 */
export function memoize<T extends (...args: any[]) => any>(fn: T): T {
  const cache = new Map();

  return ((...args: any[]) => {
    const key = JSON.stringify(args);
    if (cache.has(key)) {
      return cache.get(key);
    }
    const result = fn(...args);
    cache.set(key, result);
    return result;
  }) as T;
}

/**
 * Debounce function calls
 */
export function debounce<T extends (...args: any[]) => any>(
  fn: T,
  delay: number
): T {
  let timeoutId: NodeJS.Timeout | null = null;

  return ((...args: any[]) => {
    if (timeoutId) clearTimeout(timeoutId);
    timeoutId = setTimeout(() => {
      fn(...args);
      timeoutId = null;
    }, delay);
  }) as T;
}

/**
 * Throttle function calls
 */
export function throttle<T extends (...args: any[]) => any>(
  fn: T,
  limit: number
): T {
  let inThrottle: boolean;

  return ((...args: any[]) => {
    if (!inThrottle) {
      fn(...args);
      inThrottle = true;
      setTimeout(() => (inThrottle = false), limit);
    }
  }) as T;
}

/**
 * Request idle callback polyfill
 */
export function scheduleIdleTask(callback: () => void): void {
  if ('requestIdleCallback' in window) {
    window.requestIdleCallback(callback);
  } else {
    setTimeout(callback, 1);
  }
}

/**
 * Intersection observer for lazy loading
 */
export function observeElement(
  element: Element,
  callback: (isVisible: boolean) => void,
  options?: IntersectionObserverInit
): () => void {
  const observer = new IntersectionObserver(
    ([entry]) => {
      callback(entry.isIntersecting);
    },
    {
      threshold: 0.1,
      ...options,
    }
  );

  observer.observe(element);

  return () => observer.disconnect();
}

/**
 * Performance metrics tracker
 */
export class PerformanceTracker {
  private marks: Map<string, number> = new Map();
  private metrics: Map<string, number[]> = new Map();

  /**
   * Mark a point in time
   */
  mark(name: string): void {
    this.marks.set(name, performance.now());
  }

  /**
   * Measure time between marks
   */
  measure(name: string, startMark: string, endMark: string): number {
    const start = this.marks.get(startMark);
    const end = this.marks.get(endMark);

    if (!start || !end) {
      console.warn(`Missing mark: ${startMark} or ${endMark}`);
      return 0;
    }

    const duration = end - start;

    if (!this.metrics.has(name)) {
      this.metrics.set(name, []);
    }
    this.metrics.get(name)!.push(duration);

    return duration;
  }

  /**
   * Get average metric
   */
  getAverage(name: string): number {
    const values = this.metrics.get(name);
    if (!values || values.length === 0) return 0;
    return values.reduce((a, b) => a + b, 0) / values.length;
  }

  /**
   * Get all metrics
   */
  getMetrics(): Record<string, { average: number; count: number }> {
    const result: Record<string, { average: number; count: number }> = {};

    this.metrics.forEach((values, name) => {
      result[name] = {
        average: values.reduce((a, b) => a + b, 0) / values.length,
        count: values.length,
      };
    });

    return result;
  }

  /**
   * Clear metrics
   */
  clear(): void {
    this.marks.clear();
    this.metrics.clear();
  }

  /**
   * Log metrics
   */
  log(): void {
    console.table(this.getMetrics());
  }
}

/**
 * Global performance tracker instance
 */
export const performanceTracker = new PerformanceTracker();

/**
 * Monitor web vitals
 */
export function monitorWebVitals(): void {
  // Largest Contentful Paint
  if ('PerformanceObserver' in window) {
    try {
      const observer = new PerformanceObserver((list) => {
        for (const entry of list.getEntries()) {
          console.log('LCP:', entry.startTime);
        }
      });
      observer.observe({ entryTypes: ['largest-contentful-paint'] });
    } catch (e) {
      console.warn('LCP monitoring not supported');
    }
  }

  // First Input Delay
  if ('PerformanceObserver' in window) {
    try {
      const observer = new PerformanceObserver((list) => {
        for (const entry of list.getEntries()) {
          console.log('FID:', (entry as any).processingDuration);
        }
      });
      observer.observe({ entryTypes: ['first-input'] });
    } catch (e) {
      console.warn('FID monitoring not supported');
    }
  }

  // Cumulative Layout Shift
  if ('PerformanceObserver' in window) {
    try {
      let clsValue = 0;
      const observer = new PerformanceObserver((list) => {
        for (const entry of list.getEntries()) {
          if (!(entry as any).hadRecentInput) {
            clsValue += (entry as any).value;
            console.log('CLS:', clsValue);
          }
        }
      });
      observer.observe({ entryTypes: ['layout-shift'] });
    } catch (e) {
      console.warn('CLS monitoring not supported');
    }
  }
}

/**
 * Optimize image loading
 */
export function optimizeImageLoading(img: HTMLImageElement): void {
  // Use native lazy loading
  img.loading = 'lazy';

  // Add srcset for responsive images
  if (!img.srcset) {
    const src = img.src;
    img.srcset = `${src}?w=400 400w, ${src}?w=800 800w, ${src}?w=1200 1200w`;
  }
}

/**
 * Preload critical resources
 */
export function preloadResource(url: string, as: string): void {
  const link = document.createElement('link');
  link.rel = 'preload';
  link.as = as;
  link.href = url;
  document.head.appendChild(link);
}

/**
 * Prefetch resources
 */
export function prefetchResource(url: string): void {
  const link = document.createElement('link');
  link.rel = 'prefetch';
  link.href = url;
  document.head.appendChild(link);
}
