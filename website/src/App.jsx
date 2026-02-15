import { useState } from 'react'

export default function App() {
  const [mobileMenuOpen, setMobileMenuOpen] = useState(false)

  return (
    <div className="min-h-screen bg-white text-gray-900">
      {/* Navigation */}
      <nav className="sticky top-0 z-40 bg-white border-b border-pistachio-100">
        <div className="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex justify-between items-center py-4">
            <div className="flex items-center gap-2">
              <span className="text-2xl">🌿</span>
              <span className="text-xl font-bold text-pistachio-700">naechste</span>
            </div>
            <div className="hidden md:flex gap-8 items-center">
              <a href="#features" className="text-gray-600 hover:text-pistachio-600">Features</a>
              <a href="#getting-started" className="text-gray-600 hover:text-pistachio-600">Getting Started</a>
              <a href="#docs" className="text-gray-600 hover:text-pistachio-600">Docs</a>
              <a href="https://github.com/zeropaper/naechste" target="_blank" rel="noopener noreferrer" className="btn-primary text-sm">
                GitHub
              </a>
            </div>
            <button 
              className="md:hidden text-pistachio-600"
              onClick={() => setMobileMenuOpen(!mobileMenuOpen)}
            >
              <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 6h16M4 12h16M4 18h16" />
              </svg>
            </button>
          </div>
          {mobileMenuOpen && (
            <div className="md:hidden pb-4 border-t border-pistachio-100">
              <a href="#features" className="block py-2 text-gray-600">Features</a>
              <a href="#getting-started" className="block py-2 text-gray-600">Getting Started</a>
              <a href="#docs" className="block py-2 text-gray-600">Docs</a>
            </div>
          )}
        </div>
      </nav>

      {/* Hero Section */}
      <section className="bg-gradient-to-br from-pistachio-50 via-white to-pistachio-50 py-20 md:py-32">
        <div className="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
          <h1 className="text-4xl md:text-6xl font-bold mb-6 text-gray-900">
            Convention enforcement for modern frontends
          </h1>
          <p className="text-xl text-gray-600 mb-8 max-w-2xl mx-auto">
            <span className="font-semibold text-pistachio-700">naechste</span> brings consistent file structure and naming conventions to your React, Next.js, Vue, and Vite projects. Built with Rust, runs fast, integrates everywhere.
          </p>
          <div className="flex flex-col sm:flex-row gap-4 justify-center mb-12">
            <a href="#getting-started" className="btn-primary">Get Started</a>
            <a href="https://github.com/zeropaper/naechste" target="_blank" rel="noopener noreferrer" className="btn-secondary">
              View on GitHub →
            </a>
          </div>
          <div className="text-sm text-gray-500">
            Fast • Lightweight • Zero-dependency • Open source
          </div>
        </div>
      </section>

      {/* Problem Section */}
      <section className="py-16 md:py-24 bg-white">
        <div className="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="max-w-3xl mx-auto">
            <h2 className="text-3xl md:text-4xl font-bold mb-6 text-center text-gray-900">The Problem</h2>
            <p className="text-lg text-gray-600 mb-4">
              Unlike Angular or Ruby on Rails, React doesn't impose a file structure convention. This freedom is powerful but leads to inconsistency across teams and codebases.
            </p>
            <p className="text-lg text-gray-600">
              <span className="font-semibold text-gray-900">naechste</span> enforces structure conventions, making onboarding faster, code navigation intuitive, and allowing development tools and coding agents to scaffold files automatically.
            </p>
          </div>
        </div>
      </section>

      {/* Features Section */}
      <section id="features" className="py-16 md:py-24 bg-pistachio-50">
        <div className="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8">
          <h2 className="text-3xl md:text-4xl font-bold mb-16 text-center text-gray-900">Features</h2>
          <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-8">
            {[
              {
                icon: '⚡',
                title: 'Fast & Lightweight',
                description: 'Single Rust binary, instant linting on massive codebases.'
              },
              {
                icon: '🎯',
                title: 'Configurable Rules',
                description: 'Server-side exports, component nesting, filename consistency, file organization.'
              },
              {
                icon: '📋',
                title: 'Multiple Formats',
                description: 'JSON and human-readable output for CI/CD integration and terminal use.'
              },
              {
                icon: '🔧',
                title: 'Framework Agnostic',
                description: 'Works with Next.js, Vite, Astro, React, Vue, and any JavaScript project.'
              },
              {
                icon: '🤖',
                title: 'AI-Ready',
                description: 'Designed to work with coding agents and LLMs for smart scaffolding.'
              },
              {
                icon: '🔗',
                title: 'CI/CD Ready',
                description: 'GitHub Actions, proper exit codes, JSON output for automation.'
              }
            ].map((feature, i) => (
              <div key={i} className="bg-white rounded-lg p-6 border border-pistachio-100 hover:border-pistachio-300 hover:shadow-md transition-all">
                <div className="text-3xl mb-3">{feature.icon}</div>
                <h3 className="text-lg font-semibold text-gray-900 mb-2">{feature.title}</h3>
                <p className="text-gray-600">{feature.description}</p>
              </div>
            ))}
          </div>
        </div>
      </section>

      {/* Getting Started Section */}
      <section id="getting-started" className="py-16 md:py-24 bg-white">
        <div className="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8">
          <h2 className="text-3xl md:text-4xl font-bold mb-12 text-center text-gray-900">Get Started</h2>
          <div className="max-w-2xl mx-auto">
            <div className="bg-pistachio-50 rounded-lg p-8 border border-pistachio-200 mb-8">
              <h3 className="text-lg font-semibold text-gray-900 mb-4">Install via npm</h3>
              <code className="bg-white border border-pistachio-200 rounded px-4 py-3 block text-sm font-mono text-gray-800 mb-4">
                pnpm add -D naechste
              </code>
              <h3 className="text-lg font-semibold text-gray-900 mb-4 mt-6">Initialize</h3>
              <code className="bg-white border border-pistachio-200 rounded px-4 py-3 block text-sm font-mono text-gray-800 mb-4">
                pnpm naechste init
              </code>
              <h3 className="text-lg font-semibold text-gray-900 mb-4 mt-6">Run</h3>
              <code className="bg-white border border-pistachio-200 rounded px-4 py-3 block text-sm font-mono text-gray-800">
                pnpm naechste
              </code>
            </div>
            <p className="text-gray-600 mb-6">
              For more details, see the <a href="https://github.com/zeropaper/naechste#usage" target="_blank" rel="noopener noreferrer" className="text-pistachio-600 font-semibold hover:underline">full usage guide →</a>
            </p>
            <div className="bg-blue-50 border-l-4 border-blue-400 p-4">
              <p className="text-blue-800">
                <span className="font-semibold">💡 Tip:</span> Configuration comes next—create a <code className="bg-white px-2 py-1 rounded text-sm">naechste.json</code> or <code className="bg-white px-2 py-1 rounded text-sm">naechste.yaml</code> in your project root.
              </p>
            </div>
          </div>
        </div>
      </section>

      {/* Schemas Section */}
      <section id="docs" className="py-16 md:py-24 bg-pistachio-50">
        <div className="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
          <h2 className="text-3xl md:text-4xl font-bold mb-6 text-gray-900">Configuration Schemas</h2>
          <p className="text-lg text-gray-600 mb-8 max-w-2xl mx-auto">
            naechste configuration is validated against JSON schemas. Use them in your IDE for auto-complete and validation.
          </p>
          <div className="grid md:grid-cols-2 gap-6 max-w-2xl mx-auto">
            <a href="/docs/schemas/naechste.json" target="_blank" rel="noopener noreferrer" className="bg-white border-2 border-pistachio-200 rounded-lg p-6 hover:border-pistachio-500 transition-colors">
              <h3 className="text-lg font-semibold text-pistachio-700 mb-2">Latest Schema</h3>
              <p className="text-sm text-gray-600">naechste.json</p>
            </a>
            <a href="/docs/schemas/naechste-0.1.2-beta.2.json" target="_blank" rel="noopener noreferrer" className="bg-white border-2 border-pistachio-200 rounded-lg p-6 hover:border-pistachio-500 transition-colors">
              <h3 className="text-lg font-semibold text-pistachio-700 mb-2">v0.1.2-beta.2</h3>
              <p className="text-sm text-gray-600">naechste-0.1.2-beta.2.json</p>
            </a>
          </div>
        </div>
      </section>

      {/* CTA Section */}
      <section className="bg-gradient-to-r from-pistachio-600 to-pistachio-500 text-white py-16 md:py-20">
        <div className="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
          <h2 className="text-3xl md:text-4xl font-bold mb-4">Ready to enforce structure?</h2>
          <p className="text-lg mb-8 opacity-90">Join teams using naechste to keep their codebases consistent.</p>
          <a href="https://github.com/zeropaper/naechste" target="_blank" rel="noopener noreferrer" className="inline-block px-8 py-3 bg-white text-pistachio-600 font-semibold rounded-lg hover:bg-pistachio-50 transition-colors">
            Get Started on GitHub →
          </a>
        </div>
      </section>

      {/* Footer */}
      <footer className="bg-gray-900 text-gray-400 py-12">
        <div className="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="grid md:grid-cols-3 gap-8 mb-8">
            <div>
              <div className="flex items-center gap-2 mb-4">
                <span className="text-2xl">🌿</span>
                <span className="text-lg font-bold text-white">naechste</span>
              </div>
              <p className="text-sm">File structure linter for modern frontends.</p>
            </div>
            <div>
              <h4 className="font-semibold text-white mb-4">Project</h4>
              <ul className="space-y-2 text-sm">
                <li><a href="https://github.com/zeropaper/naechste" target="_blank" rel="noopener noreferrer" className="hover:text-pistachio-400">GitHub</a></li>
                <li><a href="https://github.com/zeropaper/naechste/issues" target="_blank" rel="noopener noreferrer" className="hover:text-pistachio-400">Issues</a></li>
                <li><a href="https://github.com/zeropaper/naechste/releases" target="_blank" rel="noopener noreferrer" className="hover:text-pistachio-400">Releases</a></li>
              </ul>
            </div>
            <div>
              <h4 className="font-semibold text-white mb-4">Community</h4>
              <ul className="space-y-2 text-sm">
                <li><a href="https://github.com/zeropaper/naechste/discussions" target="_blank" rel="noopener noreferrer" className="hover:text-pistachio-400">Discussions</a></li>
                <li><a href="https://github.com/zeropaper/naechste/blob/main/CONTRIBUTING.md" target="_blank" rel="noopener noreferrer" className="hover:text-pistachio-400">Contribute</a></li>
                <li><a href="https://github.com/zeropaper/naechste/blob/main/LICENSE" target="_blank" rel="noopener noreferrer" className="hover:text-pistachio-400">License (MIT)</a></li>
              </ul>
            </div>
          </div>
          <div className="border-t border-gray-800 pt-8 text-center text-sm">
            <p>&copy; 2026 naechste contributors. Built with Rust and ❤️</p>
          </div>
        </div>
      </footer>
    </div>
  )
}
