/**
 * Responsive layout system for Chatapp.
 * Provides utilities for responsive design and layout management.
 */

export type BreakpointName = 'xs' | 'sm' | 'md' | 'lg' | 'xl' | '2xl';

export interface Breakpoints {
  xs: number;
  sm: number;
  md: number;
  lg: number;
  xl: number;
  '2xl': number;
}

export interface ResponsiveValue<T> {
  xs?: T;
  sm?: T;
  md?: T;
  lg?: T;
  xl?: T;
  '2xl'?: T;
}

/**
 * Default breakpoints (pixels)
 */
export const DEFAULT_BREAKPOINTS: Breakpoints = {
  xs: 0,
  sm: 640,
  md: 768,
  lg: 1024,
  xl: 1280,
  '2xl': 1536,
};

/**
 * Get current breakpoint
 */
export function getCurrentBreakpoint(breakpoints: Breakpoints = DEFAULT_BREAKPOINTS): BreakpointName {
  const width = typeof window !== 'undefined' ? window.innerWidth : 0;

  if (width >= breakpoints['2xl']) return '2xl';
  if (width >= breakpoints.xl) return 'xl';
  if (width >= breakpoints.lg) return 'lg';
  if (width >= breakpoints.md) return 'md';
  if (width >= breakpoints.sm) return 'sm';
  return 'xs';
}

/**
 * Get responsive value based on current breakpoint
 */
export function getResponsiveValue<T>(
  value: T | ResponsiveValue<T>,
  breakpoint: BreakpointName
): T {
  if (typeof value !== 'object' || value === null) {
    return value as T;
  }

  const responsiveValue = value as ResponsiveValue<T>;

  // Try to get value for current breakpoint or fallback to smaller breakpoints
  const breakpointOrder: BreakpointName[] = ['xs', 'sm', 'md', 'lg', 'xl', '2xl'];
  const currentIndex = breakpointOrder.indexOf(breakpoint);

  for (let i = currentIndex; i >= 0; i--) {
    const bp = breakpointOrder[i];
    if (responsiveValue[bp] !== undefined) {
      return responsiveValue[bp]!;
    }
  }

  // Fallback to first defined value
  for (const bp of breakpointOrder) {
    if (responsiveValue[bp] !== undefined) {
      return responsiveValue[bp]!;
    }
  }

  return undefined as any;
}

/**
 * Generate CSS media query
 */
export function generateMediaQuery(breakpoint: BreakpointName, breakpoints: Breakpoints = DEFAULT_BREAKPOINTS): string {
  return `@media (min-width: ${breakpoints[breakpoint]}px)`;
}

/**
 * Grid layout utilities
 */
export interface GridConfig {
  columns: ResponsiveValue<number>;
  gap: ResponsiveValue<string>;
  autoFlow?: 'row' | 'column' | 'dense';
}

export function generateGridStyles(config: GridConfig, breakpoint: BreakpointName): React.CSSProperties {
  const columns = getResponsiveValue(config.columns, breakpoint);
  const gap = getResponsiveValue(config.gap, breakpoint);

  return {
    display: 'grid',
    gridTemplateColumns: `repeat(${columns}, 1fr)`,
    gap,
    gridAutoFlow: config.autoFlow,
  };
}

/**
 * Flex layout utilities
 */
export interface FlexConfig {
  direction: ResponsiveValue<'row' | 'column'>;
  justify: ResponsiveValue<'start' | 'center' | 'end' | 'between' | 'around'>;
  align: ResponsiveValue<'start' | 'center' | 'end' | 'stretch'>;
  gap: ResponsiveValue<string>;
  wrap?: boolean;
}

export function generateFlexStyles(config: FlexConfig, breakpoint: BreakpointName): React.CSSProperties {
  const direction = getResponsiveValue(config.direction, breakpoint);
  const justify = getResponsiveValue(config.justify, breakpoint);
  const align = getResponsiveValue(config.align, breakpoint);
  const gap = getResponsiveValue(config.gap, breakpoint);

  const justifyMap: Record<string, string> = {
    start: 'flex-start',
    center: 'center',
    end: 'flex-end',
    between: 'space-between',
    around: 'space-around',
  };

  const alignMap: Record<string, string> = {
    start: 'flex-start',
    center: 'center',
    end: 'flex-end',
    stretch: 'stretch',
  };

  return {
    display: 'flex',
    flexDirection: direction,
    justifyContent: justifyMap[justify],
    alignItems: alignMap[align],
    gap,
    flexWrap: config.wrap ? 'wrap' : 'nowrap',
  };
}

/**
 * Container query support
 */
export interface ContainerQueryConfig {
  name?: string;
  type?: 'size' | 'inline-size';
}

export function generateContainerStyles(config: ContainerQueryConfig): React.CSSProperties {
  return {
    containerName: config.name,
    containerType: config.type || 'inline-size',
  } as any;
}

/**
 * Responsive hook for listening to breakpoint changes
 */
export function useResponsiveBreakpoint(
  callback: (breakpoint: BreakpointName) => void,
  breakpoints: Breakpoints = DEFAULT_BREAKPOINTS
): (() => void) | undefined {
  if (typeof window === 'undefined') return;

  const handleResize = () => {
    const breakpoint = getCurrentBreakpoint(breakpoints);
    callback(breakpoint);
  };

  // Initial call
  handleResize();

  // Listen to resize events
  window.addEventListener('resize', handleResize);

  return () => {
    window.removeEventListener('resize', handleResize);
  };
}

/**
 * Common responsive patterns
 */
export const RESPONSIVE_PATTERNS = {
  // Mobile-first grid
  mobileFirstGrid: (columns: number): GridConfig => ({
    columns: {
      xs: 1,
      sm: 2,
      md: 3,
      lg: 4,
      xl: columns,
    },
    gap: {
      xs: '0.5rem',
      sm: '1rem',
      md: '1.5rem',
      lg: '2rem',
    },
  }),

  // Responsive padding
  responsivePadding: {
    xs: '0.5rem',
    sm: '1rem',
    md: '1.5rem',
    lg: '2rem',
    xl: '3rem',
  },

  // Responsive font sizes
  responsiveFontSize: {
    xs: '0.875rem',
    sm: '1rem',
    md: '1.125rem',
    lg: '1.25rem',
    xl: '1.5rem',
  },

  // Responsive spacing
  responsiveSpacing: {
    xs: '0.5rem',
    sm: '1rem',
    md: '1.5rem',
    lg: '2rem',
    xl: '3rem',
  },
};
