/**
 * Data visualization components for Chatapp.
 * Supports line charts, bar charts, scatter plots, and more.
 */

import React from 'react';

export interface ChartData {
  labels: string[];
  datasets: Dataset[];
}

export interface Dataset {
  label: string;
  data: number[];
  borderColor?: string;
  backgroundColor?: string;
  fill?: boolean;
  tension?: number;
}

export interface ChartConfig {
  type: 'line' | 'bar' | 'scatter' | 'pie' | 'doughnut';
  data: ChartData;
  options?: ChartOptions;
}

export interface ChartOptions {
  responsive?: boolean;
  maintainAspectRatio?: boolean;
  plugins?: {
    legend?: {
      display?: boolean;
      position?: 'top' | 'bottom' | 'left' | 'right';
    };
    title?: {
      display?: boolean;
      text?: string;
    };
  };
  scales?: {
    y?: {
      beginAtZero?: boolean;
      max?: number;
      min?: number;
    };
    x?: {
      display?: boolean;
    };
  };
}

/**
 * Simple line chart component
 */
export function LineChart({ config }: { config: ChartConfig }) {
  const { data, options = {} } = config;
  const height = 300;
  const width = 600;
  const padding = 40;

  // Calculate scales
  const allValues = data.datasets.flatMap((d) => d.data);
  const maxValue = Math.max(...allValues);
  const minValue = Math.min(...allValues);
  const range = maxValue - minValue || 1;

  const xStep = (width - 2 * padding) / (data.labels.length - 1 || 1);
  const yScale = (height - 2 * padding) / range;

  const colors = [
    '#0f62fe',
    '#24a148',
    '#da1e28',
    '#f1c21b',
    '#0043ce',
    '#8a3ffc',
  ];

  return (
    <div style={{ margin: '1rem 0', overflowX: 'auto' }}>
      <svg
        width={width}
        height={height}
        style={{
          border: '1px solid #e0e0e0',
          borderRadius: '4px',
          backgroundColor: 'white',
        }}
      >
        {/* Grid lines */}
        {[0, 0.25, 0.5, 0.75, 1].map((ratio, i) => {
          const y = height - padding - ratio * (height - 2 * padding);
          return (
            <line
              key={`grid-${i}`}
              x1={padding}
              y1={y}
              x2={width - padding}
              y2={y}
              stroke="#f0f0f0"
              strokeWidth="1"
            />
          );
        })}

        {/* Axes */}
        <line x1={padding} y1={padding} x2={padding} y2={height - padding} stroke="#333" strokeWidth="2" />
        <line x1={padding} y1={height - padding} x2={width - padding} y2={height - padding} stroke="#333" strokeWidth="2" />

        {/* Y-axis labels */}
        {[0, 0.25, 0.5, 0.75, 1].map((ratio, i) => {
          const y = height - padding - ratio * (height - 2 * padding);
          const value = minValue + ratio * range;
          return (
            <text
              key={`y-label-${i}`}
              x={padding - 10}
              y={y + 4}
              textAnchor="end"
              fontSize="12"
              fill="#666"
            >
              {value.toFixed(0)}
            </text>
          );
        })}

        {/* X-axis labels */}
        {data.labels.map((label, i) => {
          const x = padding + (i / (data.labels.length - 1 || 1)) * (width - 2 * padding);
          return (
            <text
              key={`x-label-${i}`}
              x={x}
              y={height - padding + 20}
              textAnchor="middle"
              fontSize="12"
              fill="#666"
            >
              {label}
            </text>
          );
        })}

        {/* Data lines */}
        {data.datasets.map((dataset, datasetIndex) => {
          const points = dataset.data.map((value, i) => {
            const x = padding + (i / (data.labels.length - 1 || 1)) * (width - 2 * padding);
            const y = height - padding - ((value - minValue) / range) * (height - 2 * padding);
            return { x, y };
          });

          const pathData = points.map((p, i) => `${i === 0 ? 'M' : 'L'} ${p.x} ${p.y}`).join(' ');
          const color = colors[datasetIndex % colors.length];

          return (
            <g key={`dataset-${datasetIndex}`}>
              {/* Line */}
              <path d={pathData} stroke={color} strokeWidth="2" fill="none" />

              {/* Points */}
              {points.map((p, i) => (
                <circle key={`point-${i}`} cx={p.x} cy={p.y} r="4" fill={color} />
              ))}
            </g>
          );
        })}

        {/* Legend */}
        {data.datasets.map((dataset, i) => {
          const y = 20 + i * 20;
          const color = colors[i % colors.length];
          return (
            <g key={`legend-${i}`}>
              <rect x={width - 150} y={y - 10} width="12" height="12" fill={color} />
              <text x={width - 135} y={y} fontSize="12" fill="#333">
                {dataset.label}
              </text>
            </g>
          );
        })}
      </svg>
    </div>
  );
}

/**
 * Simple bar chart component
 */
export function BarChart({ config }: { config: ChartConfig }) {
  const { data, options = {} } = config;
  const height = 300;
  const width = 600;
  const padding = 40;

  const allValues = data.datasets.flatMap((d) => d.data);
  const maxValue = Math.max(...allValues);
  const minValue = Math.min(...allValues);
  const range = maxValue - minValue || 1;

  const barWidth = (width - 2 * padding) / data.labels.length / data.datasets.length;
  const yScale = (height - 2 * padding) / range;

  const colors = [
    '#0f62fe',
    '#24a148',
    '#da1e28',
    '#f1c21b',
    '#0043ce',
    '#8a3ffc',
  ];

  return (
    <div style={{ margin: '1rem 0', overflowX: 'auto' }}>
      <svg
        width={width}
        height={height}
        style={{
          border: '1px solid #e0e0e0',
          borderRadius: '4px',
          backgroundColor: 'white',
        }}
      >
        {/* Axes */}
        <line x1={padding} y1={padding} x2={padding} y2={height - padding} stroke="#333" strokeWidth="2" />
        <line x1={padding} y1={height - padding} x2={width - padding} y2={height - padding} stroke="#333" strokeWidth="2" />

        {/* Bars */}
        {data.labels.map((label, labelIndex) => {
          const groupX = padding + (labelIndex + 0.5) * ((width - 2 * padding) / data.labels.length);

          return (
            <g key={`group-${labelIndex}`}>
              {data.datasets.map((dataset, datasetIndex) => {
                const value = dataset.data[labelIndex];
                const barHeight = ((value - minValue) / range) * (height - 2 * padding);
                const barX = groupX - (data.datasets.length * barWidth) / 2 + datasetIndex * barWidth;
                const barY = height - padding - barHeight;
                const color = colors[datasetIndex % colors.length];

                return (
                  <rect
                    key={`bar-${labelIndex}-${datasetIndex}`}
                    x={barX}
                    y={barY}
                    width={barWidth - 2}
                    height={barHeight}
                    fill={color}
                    opacity="0.8"
                  />
                );
              })}

              {/* X-axis label */}
              <text
                x={groupX}
                y={height - padding + 20}
                textAnchor="middle"
                fontSize="12"
                fill="#666"
              >
                {label}
              </text>
            </g>
          );
        })}

        {/* Y-axis labels */}
        {[0, 0.25, 0.5, 0.75, 1].map((ratio, i) => {
          const y = height - padding - ratio * (height - 2 * padding);
          const value = minValue + ratio * range;
          return (
            <text
              key={`y-label-${i}`}
              x={padding - 10}
              y={y + 4}
              textAnchor="end"
              fontSize="12"
              fill="#666"
            >
              {value.toFixed(0)}
            </text>
          );
        })}

        {/* Legend */}
        {data.datasets.map((dataset, i) => {
          const y = 20 + i * 20;
          const color = colors[i % colors.length];
          return (
            <g key={`legend-${i}`}>
              <rect x={width - 150} y={y - 10} width="12" height="12" fill={color} />
              <text x={width - 135} y={y} fontSize="12" fill="#333">
                {dataset.label}
              </text>
            </g>
          );
        })}
      </svg>
    </div>
  );
}

/**
 * Pie chart component
 */
export function PieChart({ config }: { config: ChartConfig }) {
  const { data } = config;
  const size = 300;
  const radius = 100;
  const centerX = size / 2;
  const centerY = size / 2;

  const colors = [
    '#0f62fe',
    '#24a148',
    '#da1e28',
    '#f1c21b',
    '#0043ce',
    '#8a3ffc',
  ];

  const dataset = data.datasets[0];
  const total = dataset.data.reduce((a, b) => a + b, 0);

  let currentAngle = -Math.PI / 2;
  const slices = dataset.data.map((value, i) => {
    const sliceAngle = (value / total) * 2 * Math.PI;
    const startAngle = currentAngle;
    const endAngle = currentAngle + sliceAngle;

    const x1 = centerX + radius * Math.cos(startAngle);
    const y1 = centerY + radius * Math.sin(startAngle);
    const x2 = centerX + radius * Math.cos(endAngle);
    const y2 = centerY + radius * Math.sin(endAngle);

    const largeArc = sliceAngle > Math.PI ? 1 : 0;
    const pathData = `M ${centerX} ${centerY} L ${x1} ${y1} A ${radius} ${radius} 0 ${largeArc} 1 ${x2} ${y2} Z`;

    const color = colors[i % colors.length];
    currentAngle = endAngle;

    return { pathData, color, label: data.labels[i], percentage: ((value / total) * 100).toFixed(1) };
  });

  return (
    <div style={{ margin: '1rem 0', textAlign: 'center' }}>
      <svg
        width={size}
        height={size}
        style={{
          border: '1px solid #e0e0e0',
          borderRadius: '4px',
          backgroundColor: 'white',
          display: 'inline-block',
        }}
      >
        {slices.map((slice, i) => (
          <path key={`slice-${i}`} d={slice.pathData} fill={slice.color} opacity="0.8" stroke="white" strokeWidth="2" />
        ))}
      </svg>

      {/* Legend */}
      <div style={{ marginTop: '1rem', display: 'flex', flexWrap: 'wrap', justifyContent: 'center', gap: '1rem' }}>
        {slices.map((slice, i) => (
          <div key={`legend-${i}`} style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
            <div
              style={{
                width: '12px',
                height: '12px',
                backgroundColor: slice.color,
                borderRadius: '2px',
              }}
            />
            <span style={{ fontSize: '0.875rem' }}>
              {slice.label}: {slice.percentage}%
            </span>
          </div>
        ))}
      </div>
    </div>
  );
}
