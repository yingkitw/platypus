/**
 * Advanced element rendering for additional element types.
 * Extends the basic renderer with more complex components.
 */

import React from 'react';
import { RenderContext } from './renderer';

export interface AdvancedRenderedElement {
  id: string;
  component: React.ReactNode;
}

/**
 * Render advanced/complex elements
 */
export function renderAdvancedElement(
  element: any,
  context: RenderContext
): AdvancedRenderedElement | null {
  const { id, type } = element;

  // Tabs element
  if (type?.tabs) {
    return {
      id,
      component: <TabsElement element={type.tabs} context={context} />,
    };
  }

  // Sidebar element
  if (type?.sidebar) {
    return {
      id,
      component: <SidebarElement element={type.sidebar} />,
    };
  }

  // Metric element
  if (type?.metric) {
    return {
      id,
      component: <MetricElement element={type.metric} />,
    };
  }

  // Table element
  if (type?.table) {
    return {
      id,
      component: <TableElement element={type.table} />,
    };
  }

  // Container element
  if (type?.container) {
    return {
      id,
      component: <ContainerElement element={type.container} />,
    };
  }

  // Column element
  if (type?.column) {
    return {
      id,
      component: <ColumnElement element={type.column} />,
    };
  }

  // Row element
  if (type?.row) {
    return {
      id,
      component: <RowElement element={type.row} />,
    };
  }

  return null;
}

// ============================================================================
// Advanced Layout Components
// ============================================================================

interface TabsElementProps {
  element: any;
  context: RenderContext;
}

function TabsElement({ element, context }: TabsElementProps) {
  const [activeTab, setActiveTab] = React.useState(0);
  const tabs = element.tabs || [];

  return (
    <div style={{ margin: '1rem 0' }}>
      {/* Tab buttons */}
      <div
        style={{
          display: 'flex',
          borderBottom: '2px solid #e0e0e0',
          marginBottom: '1rem',
        }}
      >
        {tabs.map((tab: any, index: number) => (
          <button
            key={index}
            onClick={() => setActiveTab(index)}
            style={{
              padding: '0.75rem 1.5rem',
              backgroundColor: activeTab === index ? '#0066cc' : 'transparent',
              color: activeTab === index ? 'white' : '#333',
              border: 'none',
              cursor: 'pointer',
              borderRadius: '4px 4px 0 0',
              marginRight: '0.5rem',
            }}
          >
            {tab.label}
          </button>
        ))}
      </div>

      {/* Tab content */}
      <div style={{ padding: '1rem' }}>
        {tabs[activeTab]?.children && (
          <div>
            {tabs[activeTab].children.map((childId: string) => (
              <div key={childId}>Child: {childId}</div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}

function SidebarElement({ element }: { element: any }) {
  return (
    <div
      style={{
        display: 'flex',
        gap: '1rem',
        margin: '1rem 0',
      }}
    >
      <aside
        style={{
          width: '250px',
          padding: '1rem',
          backgroundColor: '#f5f5f5',
          borderRadius: '4px',
          flexShrink: 0,
        }}
      >
        <h3 style={{ marginTop: 0 }}>Sidebar</h3>
        {element.children?.map((childId: string) => (
          <div key={childId} style={{ marginBottom: '0.5rem' }}>
            {childId}
          </div>
        ))}
      </aside>
      <main style={{ flex: 1 }}>Main content</main>
    </div>
  );
}

function MetricElement({ element }: { element: any }) {
  const deltaValue = element.delta ? parseFloat(element.delta) : 0;
  const isPositive = deltaValue >= 0;

  return (
    <div
      style={{
        padding: '1.5rem',
        backgroundColor: 'white',
        border: '1px solid #e0e0e0',
        borderRadius: '4px',
        margin: '1rem 0',
      }}
    >
      <p style={{ margin: '0 0 0.5rem 0', color: '#666', fontSize: '0.875rem' }}>
        {element.label}
      </p>
      <div style={{ display: 'flex', alignItems: 'baseline', gap: '1rem' }}>
        <h2 style={{ margin: 0, fontSize: '2rem' }}>{element.value}</h2>
        {element.delta && (
          <span
            style={{
              color: isPositive ? '#4caf50' : '#f44336',
              fontSize: '0.875rem',
              fontWeight: 500,
            }}
          >
            {isPositive ? '↑' : '↓'} {Math.abs(deltaValue)}%
          </span>
        )}
      </div>
    </div>
  );
}

function TableElement({ element }: { element: any }) {
  const headers = element.headers || [];
  const rows = element.rows || [];

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
            {headers.map((header: string, index: number) => (
              <th
                key={index}
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
          {rows.map((row: any, rowIndex: number) => (
            <tr
              key={rowIndex}
              style={{
                backgroundColor: rowIndex % 2 === 0 ? 'white' : '#fafafa',
              }}
            >
              {row.cells?.map((cell: string, cellIndex: number) => (
                <td
                  key={cellIndex}
                  style={{
                    padding: '0.75rem',
                    borderBottom: '1px solid #e0e0e0',
                  }}
                >
                  {cell}
                </td>
              ))}
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

function ContainerElement({ element }: { element: any }) {
  return (
    <div
      style={{
        padding: '1rem',
        backgroundColor: 'white',
        border: '1px solid #e0e0e0',
        borderRadius: '4px',
        margin: '1rem 0',
      }}
    >
      {element.children?.map((childId: string) => (
        <div key={childId} style={{ marginBottom: '0.5rem' }}>
          {childId}
        </div>
      ))}
    </div>
  );
}

function ColumnElement({ element }: { element: any }) {
  const width = element.width ? `${element.width * 100}%` : 'auto';

  return (
    <div
      style={{
        width,
        display: 'flex',
        flexDirection: 'column',
        gap: '1rem',
      }}
    >
      {element.children?.map((childId: string) => (
        <div key={childId}>{childId}</div>
      ))}
    </div>
  );
}

function RowElement({ element }: { element: any }) {
  return (
    <div
      style={{
        display: 'flex',
        gap: '1rem',
        margin: '1rem 0',
      }}
    >
      {element.children?.map((childId: string) => (
        <div key={childId} style={{ flex: 1 }}>
          {childId}
        </div>
      ))}
    </div>
  );
}
