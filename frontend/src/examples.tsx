/**
 * Example applications for Chatapp.
 * Demonstrates various features and patterns.
 */

import React, { useState } from 'react';

/**
 * Counter Example - Simple state management
 */
export function CounterExample() {
  const [count, setCount] = useState(0);

  return (
    <div style={{ padding: '2rem', maxWidth: '600px', margin: '0 auto' }}>
      <h2>Counter Example</h2>
      <p style={{ fontSize: '2rem', textAlign: 'center' }}>Count: {count}</p>
      <div style={{ display: 'flex', gap: '1rem', justifyContent: 'center' }}>
        <button onClick={() => setCount(count - 1)}>Decrease</button>
        <button onClick={() => setCount(0)}>Reset</button>
        <button onClick={() => setCount(count + 1)}>Increase</button>
      </div>
    </div>
  );
}

/**
 * Form Example - Input handling
 */
export function FormExample() {
  const [formData, setFormData] = useState({
    name: '',
    email: '',
    message: '',
  });

  const handleChange = (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
    const { name, value } = e.target;
    setFormData((prev) => ({ ...prev, [name]: value }));
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    console.log('Form submitted:', formData);
    alert(`Thank you, ${formData.name}!`);
  };

  return (
    <div style={{ padding: '2rem', maxWidth: '600px', margin: '0 auto' }}>
      <h2>Contact Form Example</h2>
      <form onSubmit={handleSubmit} style={{ display: 'flex', flexDirection: 'column', gap: '1rem' }}>
        <div>
          <label>Name:</label>
          <input
            type="text"
            name="name"
            value={formData.name}
            onChange={handleChange}
            required
            style={{ width: '100%', padding: '0.5rem', marginTop: '0.5rem' }}
          />
        </div>
        <div>
          <label>Email:</label>
          <input
            type="email"
            name="email"
            value={formData.email}
            onChange={handleChange}
            required
            style={{ width: '100%', padding: '0.5rem', marginTop: '0.5rem' }}
          />
        </div>
        <div>
          <label>Message:</label>
          <textarea
            name="message"
            value={formData.message}
            onChange={handleChange}
            rows={4}
            required
            style={{ width: '100%', padding: '0.5rem', marginTop: '0.5rem' }}
          />
        </div>
        <button type="submit" style={{ padding: '0.75rem', cursor: 'pointer' }}>
          Send Message
        </button>
      </form>
    </div>
  );
}

/**
 * Dashboard Example - Multiple widgets
 */
export function DashboardExample() {
  const [metrics, setMetrics] = useState({
    users: 1234,
    revenue: 45678,
    growth: 12.5,
  });

  return (
    <div style={{ padding: '2rem', maxWidth: '1200px', margin: '0 auto' }}>
      <h2>Dashboard Example</h2>
      <div
        style={{
          display: 'grid',
          gridTemplateColumns: 'repeat(auto-fit, minmax(250px, 1fr))',
          gap: '1.5rem',
          marginBottom: '2rem',
        }}
      >
        {/* Metric Card */}
        <div
          style={{
            padding: '1.5rem',
            backgroundColor: '#f5f5f5',
            borderRadius: '8px',
            border: '1px solid #e0e0e0',
          }}
        >
          <h3 style={{ margin: '0 0 0.5rem 0', color: '#666' }}>Total Users</h3>
          <p style={{ margin: 0, fontSize: '2rem', fontWeight: 'bold' }}>{metrics.users}</p>
          <p style={{ margin: '0.5rem 0 0 0', color: '#999', fontSize: '0.875rem' }}>↑ 5% from last month</p>
        </div>

        {/* Revenue Card */}
        <div
          style={{
            padding: '1.5rem',
            backgroundColor: '#f5f5f5',
            borderRadius: '8px',
            border: '1px solid #e0e0e0',
          }}
        >
          <h3 style={{ margin: '0 0 0.5rem 0', color: '#666' }}>Revenue</h3>
          <p style={{ margin: 0, fontSize: '2rem', fontWeight: 'bold' }}>${metrics.revenue}</p>
          <p style={{ margin: '0.5rem 0 0 0', color: '#999', fontSize: '0.875rem' }}>↑ 8% from last month</p>
        </div>

        {/* Growth Card */}
        <div
          style={{
            padding: '1.5rem',
            backgroundColor: '#f5f5f5',
            borderRadius: '8px',
            border: '1px solid #e0e0e0',
          }}
        >
          <h3 style={{ margin: '0 0 0.5rem 0', color: '#666' }}>Growth Rate</h3>
          <p style={{ margin: 0, fontSize: '2rem', fontWeight: 'bold' }}>{metrics.growth}%</p>
          <p style={{ margin: '0.5rem 0 0 0', color: '#999', fontSize: '0.875rem' }}>↑ 2% from last month</p>
        </div>
      </div>

      {/* Action Buttons */}
      <div style={{ display: 'flex', gap: '1rem' }}>
        <button onClick={() => setMetrics({ ...metrics, users: metrics.users + 100 })}>
          Add Users
        </button>
        <button onClick={() => setMetrics({ ...metrics, revenue: metrics.revenue + 1000 })}>
          Add Revenue
        </button>
        <button onClick={() => setMetrics({ users: 1234, revenue: 45678, growth: 12.5 })}>
          Reset
        </button>
      </div>
    </div>
  );
}

/**
 * Todo List Example - List management
 */
export function TodoExample() {
  const [todos, setTodos] = useState([
    { id: 1, text: 'Learn Chatapp', completed: true },
    { id: 2, text: 'Build an app', completed: false },
    { id: 3, text: 'Deploy to production', completed: false },
  ]);
  const [input, setInput] = useState('');

  const addTodo = () => {
    if (input.trim()) {
      setTodos([...todos, { id: Date.now(), text: input, completed: false }]);
      setInput('');
    }
  };

  const toggleTodo = (id: number) => {
    setTodos(todos.map((t) => (t.id === id ? { ...t, completed: !t.completed } : t)));
  };

  const deleteTodo = (id: number) => {
    setTodos(todos.filter((t) => t.id !== id));
  };

  return (
    <div style={{ padding: '2rem', maxWidth: '600px', margin: '0 auto' }}>
      <h2>Todo List Example</h2>
      <div style={{ display: 'flex', gap: '0.5rem', marginBottom: '1rem' }}>
        <input
          type="text"
          value={input}
          onChange={(e) => setInput(e.target.value)}
          onKeyPress={(e) => e.key === 'Enter' && addTodo()}
          placeholder="Add a new todo..."
          style={{ flex: 1, padding: '0.5rem' }}
        />
        <button onClick={addTodo}>Add</button>
      </div>

      <div style={{ display: 'flex', flexDirection: 'column', gap: '0.5rem' }}>
        {todos.map((todo) => (
          <div
            key={todo.id}
            style={{
              display: 'flex',
              alignItems: 'center',
              gap: '0.5rem',
              padding: '0.75rem',
              backgroundColor: '#f5f5f5',
              borderRadius: '4px',
              textDecoration: todo.completed ? 'line-through' : 'none',
              opacity: todo.completed ? 0.6 : 1,
            }}
          >
            <input
              type="checkbox"
              checked={todo.completed}
              onChange={() => toggleTodo(todo.id)}
            />
            <span style={{ flex: 1 }}>{todo.text}</span>
            <button onClick={() => deleteTodo(todo.id)} style={{ padding: '0.25rem 0.5rem' }}>
              Delete
            </button>
          </div>
        ))}
      </div>

      <p style={{ marginTop: '1rem', color: '#666' }}>
        {todos.filter((t) => t.completed).length} of {todos.length} completed
      </p>
    </div>
  );
}

/**
 * Settings Example - Configuration UI
 */
export function SettingsExample() {
  const [settings, setSettings] = useState({
    theme: 'light',
    notifications: true,
    autoSave: true,
    language: 'en',
  });

  return (
    <div style={{ padding: '2rem', maxWidth: '600px', margin: '0 auto' }}>
      <h2>Settings Example</h2>
      <div style={{ display: 'flex', flexDirection: 'column', gap: '1.5rem' }}>
        {/* Theme Setting */}
        <div>
          <label style={{ display: 'block', marginBottom: '0.5rem', fontWeight: 500 }}>
            Theme
          </label>
          <select
            value={settings.theme}
            onChange={(e) => setSettings({ ...settings, theme: e.target.value })}
            style={{ width: '100%', padding: '0.5rem' }}
          >
            <option value="light">Light</option>
            <option value="dark">Dark</option>
            <option value="auto">Auto</option>
          </select>
        </div>

        {/* Notifications Setting */}
        <div>
          <label style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
            <input
              type="checkbox"
              checked={settings.notifications}
              onChange={(e) => setSettings({ ...settings, notifications: e.target.checked })}
            />
            Enable Notifications
          </label>
        </div>

        {/* Auto-save Setting */}
        <div>
          <label style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
            <input
              type="checkbox"
              checked={settings.autoSave}
              onChange={(e) => setSettings({ ...settings, autoSave: e.target.checked })}
            />
            Auto-save Changes
          </label>
        </div>

        {/* Language Setting */}
        <div>
          <label style={{ display: 'block', marginBottom: '0.5rem', fontWeight: 500 }}>
            Language
          </label>
          <select
            value={settings.language}
            onChange={(e) => setSettings({ ...settings, language: e.target.value })}
            style={{ width: '100%', padding: '0.5rem' }}
          >
            <option value="en">English</option>
            <option value="es">Spanish</option>
            <option value="fr">French</option>
            <option value="de">German</option>
          </select>
        </div>

        {/* Save Button */}
        <button onClick={() => console.log('Settings saved:', settings)}>Save Settings</button>
      </div>
    </div>
  );
}

/**
 * Example selector component
 */
export function ExampleSelector() {
  const [selected, setSelected] = useState<'counter' | 'form' | 'dashboard' | 'todo' | 'settings'>(
    'counter'
  );

  const examples = {
    counter: { name: 'Counter', component: CounterExample },
    form: { name: 'Contact Form', component: FormExample },
    dashboard: { name: 'Dashboard', component: DashboardExample },
    todo: { name: 'Todo List', component: TodoExample },
    settings: { name: 'Settings', component: SettingsExample },
  };

  const SelectedComponent = examples[selected].component;

  return (
    <div>
      {/* Navigation */}
      <div
        style={{
          backgroundColor: '#f5f5f5',
          padding: '1rem',
          borderBottom: '1px solid #e0e0e0',
          display: 'flex',
          gap: '0.5rem',
          flexWrap: 'wrap',
        }}
      >
        {Object.entries(examples).map(([key, { name }]) => (
          <button
            key={key}
            onClick={() => setSelected(key as any)}
            style={{
              padding: '0.5rem 1rem',
              backgroundColor: selected === key ? '#0066cc' : 'white',
              color: selected === key ? 'white' : '#333',
              border: '1px solid #e0e0e0',
              borderRadius: '4px',
              cursor: 'pointer',
            }}
          >
            {name}
          </button>
        ))}
      </div>

      {/* Content */}
      <div style={{ backgroundColor: 'white' }}>
        <SelectedComponent />
      </div>
    </div>
  );
}
