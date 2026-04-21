export default function ExtensionsPanel() {
  return (
    <div className="h-full overflow-auto p-6">
      <div className="max-w-4xl mx-auto">
        <div className="flex items-center gap-3 mb-8">
          <div className="p-2 rounded-lg bg-accent-soft-bg text-accent">
            <span className="text-2xl">🧩</span>
          </div>
          <div>
            <h1 className="text-2xl font-bold text-primary">Extensions</h1>
            <p className="text-muted">Enhance your Zaroxi experience with extensions</p>
          </div>
        </div>
        
        <div className="bg-panel rounded-xl border border-border p-6">
          <h2 className="text-xl font-semibold text-primary mb-4">Coming Soon</h2>
          <p className="text-muted mb-4">
            The extensions marketplace is under development. You'll be able to browse, install, and manage extensions for:
          </p>
          <ul className="space-y-2 text-muted">
            <li>• Language support (Python, JavaScript, Go, etc.)</li>
            <li>• Themes and UI customizations</li>
            <li>• Productivity tools and integrations</li>
            <li>• AI assistants and code analysis</li>
          </ul>
          <div className="mt-6 p-4 bg-muted rounded-lg">
            <p className="text-sm text-primary">
              <strong>Note:</strong> Extension support is planned for a future release. Stay tuned!
            </p>
          </div>
        </div>
      </div>
    </div>
  );
}
