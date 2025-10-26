/**
 * Caching system for Chatapp.
 * Provides decorators and utilities for caching data and computations.
 */

export interface CacheEntry<T> {
  value: T;
  timestamp: number;
  ttl?: number;
}

export interface CacheOptions {
  ttl?: number; // Time to live in milliseconds
  maxSize?: number; // Maximum number of entries
}

/**
 * Simple in-memory cache
 */
export class Cache<T = any> {
  private entries: Map<string, CacheEntry<T>> = new Map();
  private options: CacheOptions;

  constructor(options: CacheOptions = {}) {
    this.options = {
      ttl: options.ttl || 5 * 60 * 1000, // 5 minutes default
      maxSize: options.maxSize || 100,
    };
  }

  /**
   * Get value from cache
   */
  get(key: string): T | null {
    const entry = this.entries.get(key);

    if (!entry) return null;

    // Check if expired
    if (entry.ttl && Date.now() - entry.timestamp > entry.ttl) {
      this.entries.delete(key);
      return null;
    }

    return entry.value;
  }

  /**
   * Set value in cache
   */
  set(key: string, value: T, ttl?: number): void {
    // Evict oldest entry if cache is full
    if (this.entries.size >= this.options.maxSize!) {
      const firstKey = this.entries.keys().next().value;
      if (firstKey) this.entries.delete(firstKey);
    }

    this.entries.set(key, {
      value,
      timestamp: Date.now(),
      ttl: ttl || this.options.ttl,
    });
  }

  /**
   * Check if key exists and is valid
   */
  has(key: string): boolean {
    return this.get(key) !== null;
  }

  /**
   * Delete entry
   */
  delete(key: string): void {
    this.entries.delete(key);
  }

  /**
   * Clear all entries
   */
  clear(): void {
    this.entries.clear();
  }

  /**
   * Get cache size
   */
  size(): number {
    return this.entries.size;
  }

  /**
   * Clean expired entries
   */
  cleanup(): void {
    const now = Date.now();
    const keysToDelete: string[] = [];

    this.entries.forEach((entry, key) => {
      if (entry.ttl && now - entry.timestamp > entry.ttl) {
        keysToDelete.push(key);
      }
    });

    keysToDelete.forEach((key) => this.entries.delete(key));
  }
}

/**
 * Global cache instance
 */
const globalCache = new Cache();

/**
 * Memoize a function with caching
 */
export function memoize<T extends (...args: any[]) => any>(
  fn: T,
  options: CacheOptions = {}
): T {
  const cache = new Cache(options);

  return ((...args: any[]) => {
    const key = JSON.stringify(args);
    const cached = cache.get(key);

    if (cached !== null) {
      return cached;
    }

    const result = fn(...args);
    cache.set(key, result);
    return result;
  }) as T;
}

/**
 * Cache decorator for async functions
 */
export function cacheAsync<T extends (...args: any[]) => Promise<any>>(
  fn: T,
  options: CacheOptions = {}
): T {
  const cache = new Cache(options);

  return (async (...args: any[]) => {
    const key = JSON.stringify(args);
    const cached = cache.get(key);

    if (cached !== null) {
      return cached;
    }

    const result = await fn(...args);
    cache.set(key, result);
    return result;
  }) as T;
}

/**
 * Get or compute a value
 */
export async function getOrCompute<T>(
  key: string,
  compute: () => Promise<T>,
  options: CacheOptions = {}
): Promise<T> {
  const cached = globalCache.get(key);
  if (cached !== null) {
    return cached as T;
  }

  const result = await compute();
  globalCache.set(key, result, options.ttl);
  return result;
}

/**
 * Set a cached value
 */
export function setCached<T>(key: string, value: T, ttl?: number): void {
  globalCache.set(key, value, ttl);
}

/**
 * Get a cached value
 */
export function getCached<T>(key: string): T | null {
  return globalCache.get(key) as T | null;
}

/**
 * Clear cache
 */
export function clearCache(): void {
  globalCache.clear();
}

/**
 * Clear specific cache entry
 */
export function clearCacheEntry(key: string): void {
  globalCache.delete(key);
}

/**
 * Local storage cache with fallback
 */
export class PersistentCache<T = any> {
  private memoryCache: Cache<T>;
  private storageKey: string;

  constructor(storageKey: string, options: CacheOptions = {}) {
    this.storageKey = `cache:${storageKey}`;
    this.memoryCache = new Cache(options);
  }

  /**
   * Get value from cache
   */
  get(key: string): T | null {
    // Try memory cache first
    const memCached = this.memoryCache.get(key);
    if (memCached !== null) return memCached;

    // Try local storage
    if (typeof localStorage !== 'undefined') {
      try {
        const stored = localStorage.getItem(`${this.storageKey}:${key}`);
        if (stored) {
          const data = JSON.parse(stored);
          // Restore to memory cache
          this.memoryCache.set(key, data.value);
          return data.value;
        }
      } catch (error) {
        console.warn('Failed to read from localStorage:', error);
      }
    }

    return null;
  }

  /**
   * Set value in cache
   */
  set(key: string, value: T, ttl?: number): void {
    // Store in memory cache
    this.memoryCache.set(key, value, ttl);

    // Store in local storage
    if (typeof localStorage !== 'undefined') {
      try {
        localStorage.setItem(
          `${this.storageKey}:${key}`,
          JSON.stringify({ value, timestamp: Date.now(), ttl })
        );
      } catch (error) {
        console.warn('Failed to write to localStorage:', error);
      }
    }
  }

  /**
   * Clear cache
   */
  clear(): void {
    this.memoryCache.clear();

    if (typeof localStorage !== 'undefined') {
      try {
        const keys = Object.keys(localStorage);
        keys.forEach((key) => {
          if (key.startsWith(this.storageKey)) {
            localStorage.removeItem(key);
          }
        });
      } catch (error) {
        console.warn('Failed to clear localStorage:', error);
      }
    }
  }
}

/**
 * Session storage cache
 */
export class SessionCache<T = any> extends PersistentCache<T> {
  constructor(storageKey: string, options: CacheOptions = {}) {
    super(storageKey, options);
  }
}
