/**
 * Element rendering engine for converting proto messages to React components.
 */

import React from 'react';

export interface RenderedElement {
  id: string;
  component: React.ReactNode;
}

export interface RenderContext {
  onWidgetChange: (key: string, value: unknown) => void;
  onButtonClick: (key: string) => void;
}

/**
 * Render a single element based on its type
 */
export function renderElement(
  element: any,
  context: RenderContext
): RenderedElement {
  const { id, type } = element;

  // Determine element type and render accordingly
  if (type?.text) {
    return {
      id,
      component: <TextElement element={type.text} />,
    };
  }

  if (type?.markdown) {
    return {
      id,
      component: <MarkdownElement element={type.markdown} />,
    };
  }

  if (type?.heading) {
    return {
      id,
      component: <HeadingElement element={type.heading} />,
    };
  }

  if (type?.button) {
    return {
      id,
      component: (
        <ButtonElement
          element={type.button}
          onClick={() => context.onButtonClick(type.button.key)}
        />
      ),
    };
  }

  if (type?.textInput) {
    return {
      id,
      component: (
        <TextInputElement
          element={type.textInput}
          onChange={(value) => context.onWidgetChange(type.textInput.key, value)}
        />
      ),
    };
  }

  if (type?.slider) {
    return {
      id,
      component: (
        <SliderElement
          element={type.slider}
          onChange={(value) => context.onWidgetChange(type.slider.key, value)}
        />
      ),
    };
  }

  if (type?.checkbox) {
    return {
      id,
      component: (
        <CheckboxElement
          element={type.checkbox}
          onChange={(value) => context.onWidgetChange(type.checkbox.key, value)}
        />
      ),
    };
  }

  if (type?.selectbox) {
    return {
      id,
      component: (
        <SelectboxElement
          element={type.selectbox}
          onChange={(value) => context.onWidgetChange(type.selectbox.key, value)}
        />
      ),
    };
  }

  if (type?.success) {
    return {
      id,
      component: <SuccessElement element={type.success} />,
    };
  }

  if (type?.error) {
    return {
      id,
      component: <ErrorElement element={type.error} />,
    };
  }

  if (type?.warning) {
    return {
      id,
      component: <WarningElement element={type.warning} />,
    };
  }

  if (type?.info) {
    return {
      id,
      component: <InfoElement element={type.info} />,
    };
  }

  if (type?.progress) {
    return {
      id,
      component: <ProgressElement element={type.progress} />,
    };
  }

  if (type?.divider) {
    return {
      id,
      component: <DividerElement />,
    };
  }

  if (type?.empty) {
    return {
      id,
      component: <div style={{ height: '1rem' }} />,
    };
  }

  // Media elements
  if (type?.image) {
    return {
      id,
      component: <ImageElement element={type.image} />,
    };
  }

  if (type?.audio) {
    return {
      id,
      component: <AudioElement element={type.audio} />,
    };
  }

  if (type?.video) {
    return {
      id,
      component: <VideoElement element={type.video} />,
    };
  }

  // Data display
  if (type?.json) {
    return {
      id,
      component: <JsonElement element={type.json} />,
    };
  }

  if (type?.dataframe) {
    return {
      id,
      component: <DataframeElement element={type.dataframe} />,
    };
  }

  // Fallback for unknown types
  return {
    id,
    component: <div>Unknown element type</div>,
  };
}

// ============================================================================
// Display Elements
// ============================================================================

function TextElement({ element }: { element: any }) {
  return <p style={{ margin: '0.5rem 0' }}>{element.value}</p>;
}

function MarkdownElement({ element }: { element: any }) {
  return <div style={{ margin: '0.5rem 0' }}>{element.value}</div>;
}

function HeadingElement({ element }: { element: any }) {
  const level = element.level || 1;
  const HeadingTag = `h${level}` as keyof JSX.IntrinsicElements;
  return (
    <HeadingTag style={{ margin: '1rem 0 0.5rem 0' }}>
      {element.value}
    </HeadingTag>
  );
}

function DividerElement() {
  return <hr style={{ margin: '1rem 0', border: 'none', borderTop: '1px solid #ccc' }} />;
}

// ============================================================================
// Input Widgets
// ============================================================================

interface WidgetProps {
  element: any;
  onChange?: (value: unknown) => void;
  onClickHandler?: () => void;
}

function TextInputElement({ element, onChange }: WidgetProps) {
  return (
    <div style={{ margin: '0.5rem 0' }}>
      <label style={{ display: 'block', marginBottom: '0.25rem', fontWeight: 500 }}>
        {element.label}
      </label>
      <input
        type="text"
        defaultValue={element.value}
        onChange={(e) => onChange?.(e.target.value)}
        style={{
          width: '100%',
          padding: '0.5rem',
          border: '1px solid #ccc',
          borderRadius: '4px',
          fontSize: '1rem',
        }}
      />
    </div>
  );
}

function SliderElement({ element, onChange }: WidgetProps) {
  return (
    <div style={{ margin: '0.5rem 0' }}>
      <label style={{ display: 'block', marginBottom: '0.25rem', fontWeight: 500 }}>
        {element.label}: {element.value}
      </label>
      <input
        type="range"
        min={element.min}
        max={element.max}
        defaultValue={element.value}
        onChange={(e) => onChange?.(parseFloat(e.target.value))}
        style={{ width: '100%' }}
      />
    </div>
  );
}

function CheckboxElement({ element, onChange }: WidgetProps) {
  return (
    <div style={{ margin: '0.5rem 0' }}>
      <label style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
        <input
          type="checkbox"
          defaultChecked={element.value}
          onChange={(e) => onChange?.(e.target.checked)}
        />
        {element.label}
      </label>
    </div>
  );
}

function SelectboxElement({ element, onChange }: WidgetProps) {
  return (
    <div style={{ margin: '0.5rem 0' }}>
      <label style={{ display: 'block', marginBottom: '0.25rem', fontWeight: 500 }}>
        {element.label}
      </label>
      <select
        defaultValue={element.value}
        onChange={(e) => onChange?.(e.target.value)}
        style={{
          width: '100%',
          padding: '0.5rem',
          border: '1px solid #ccc',
          borderRadius: '4px',
          fontSize: '1rem',
        }}
      >
        {element.options?.map((opt: string) => (
          <option key={opt} value={opt}>
            {opt}
          </option>
        ))}
      </select>
    </div>
  );
}

function ButtonElement({ element, onClickHandler }: WidgetProps & { onClickHandler?: () => void }) {
  return (
    <button
      onClick={onClickHandler}
      style={{
        padding: '0.5rem 1rem',
        backgroundColor: '#0066cc',
        color: 'white',
        border: 'none',
        borderRadius: '4px',
        fontSize: '1rem',
        cursor: 'pointer',
        margin: '0.5rem 0',
      }}
    >
      {element.label}
    </button>
  );
}

// ============================================================================
// Feedback Elements
// ============================================================================

function SuccessElement({ element }: { element: any }) {
  return (
    <div
      style={{
        padding: '1rem',
        backgroundColor: '#d4edda',
        color: '#155724',
        borderRadius: '4px',
        margin: '0.5rem 0',
        border: '1px solid #c3e6cb',
      }}
    >
      ✓ {element.message}
    </div>
  );
}

function ErrorElement({ element }: { element: any }) {
  return (
    <div
      style={{
        padding: '1rem',
        backgroundColor: '#f8d7da',
        color: '#721c24',
        borderRadius: '4px',
        margin: '0.5rem 0',
        border: '1px solid #f5c6cb',
      }}
    >
      ✕ {element.message}
    </div>
  );
}

function WarningElement({ element }: { element: any }) {
  return (
    <div
      style={{
        padding: '1rem',
        backgroundColor: '#fff3cd',
        color: '#856404',
        borderRadius: '4px',
        margin: '0.5rem 0',
        border: '1px solid #ffeaa7',
      }}
    >
      ⚠ {element.message}
    </div>
  );
}

function InfoElement({ element }: { element: any }) {
  return (
    <div
      style={{
        padding: '1rem',
        backgroundColor: '#d1ecf1',
        color: '#0c5460',
        borderRadius: '4px',
        margin: '0.5rem 0',
        border: '1px solid #bee5eb',
      }}
    >
      ℹ {element.message}
    </div>
  );
}

function ProgressElement({ element }: { element: any }) {
  const percentage = Math.min(100, Math.max(0, element.value * 100));
  return (
    <div style={{ margin: '0.5rem 0' }}>
      <div
        style={{
          width: '100%',
          height: '24px',
          backgroundColor: '#e0e0e0',
          borderRadius: '4px',
          overflow: 'hidden',
        }}
      >
        <div
          style={{
            height: '100%',
            width: `${percentage}%`,
            backgroundColor: '#4caf50',
            transition: 'width 0.3s ease',
          }}
        />
      </div>
      <p style={{ margin: '0.25rem 0', fontSize: '0.875rem', color: '#666' }}>
        {Math.round(percentage)}%
      </p>
    </div>
  );
}

// ============================================================================
// Media Elements
// ============================================================================

function ImageElement({ element }: { element: any }) {
  return (
    <figure style={{ margin: '1rem 0', textAlign: 'center' }}>
      <img
        src={element.src}
        alt={element.caption || 'Image'}
        style={{
          maxWidth: element.width ? `${element.width}px` : '100%',
          height: 'auto',
          borderRadius: '4px',
        }}
      />
      {element.caption && (
        <figcaption style={{ marginTop: '0.5rem', color: '#666', fontSize: '0.875rem' }}>
          {element.caption}
        </figcaption>
      )}
    </figure>
  );
}

function AudioElement({ element }: { element: any }) {
  return (
    <audio
      controls
      style={{ width: '100%', margin: '1rem 0' }}
    >
      <source src={element.src} />
      Your browser does not support the audio element.
    </audio>
  );
}

function VideoElement({ element }: { element: any }) {
  return (
    <video
      controls
      style={{ width: '100%', maxWidth: '600px', margin: '1rem 0', borderRadius: '4px' }}
    >
      <source src={element.src} />
      Your browser does not support the video element.
    </video>
  );
}

function JsonElement({ element }: { element: any }) {
  return (
    <pre
      style={{
        backgroundColor: '#f5f5f5',
        padding: '1rem',
        borderRadius: '4px',
        overflow: 'auto',
        fontSize: '0.875rem',
        margin: '1rem 0',
      }}
    >
      <code>{JSON.stringify(element.value, null, 2)}</code>
    </pre>
  );
}

function DataframeElement({ element }: { element: any }) {
  try {
    const data = JSON.parse(element.data);
    const headers = Object.keys(data[0] || {});

    return (
      <div style={{ margin: '1rem 0', overflowX: 'auto' }}>
        <table
          style={{
            width: '100%',
            borderCollapse: 'collapse',
            border: '1px solid #e0e0e0',
          }}
        >
          <thead>
            <tr style={{ backgroundColor: '#f5f5f5' }}>
              {headers.map((header: string) => (
                <th
                  key={header}
                  style={{
                    padding: '0.75rem',
                    textAlign: 'left',
                    borderBottom: '2px solid #e0e0e0',
                    fontWeight: 600,
                  }}
                >
                  {header}
                </th>
              ))}
            </tr>
          </thead>
          <tbody>
            {data.map((row: any, rowIndex: number) => (
              <tr
                key={rowIndex}
                style={{
                  backgroundColor: rowIndex % 2 === 0 ? 'white' : '#fafafa',
                }}
              >
                {headers.map((header: string) => (
                  <td
                    key={`${rowIndex}-${header}`}
                    style={{
                      padding: '0.75rem',
                      borderBottom: '1px solid #e0e0e0',
                    }}
                  >
                    {String(row[header] || '')}
                  </td>
                ))}
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    );
  } catch (error) {
    return <div>Error rendering dataframe: {String(error)}</div>;
  }
}
