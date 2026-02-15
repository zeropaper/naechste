import { Link } from 'react-router-dom'
import { ArrowRight, Zap, Target, FileText, Cog } from 'lucide-react'

export default function Home() {
  return (
    <div>
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
            <Link to="/docs/getting-started" className="btn-primary">Get Started</Link>
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
      <section  className="py-16 md:py-24 bg-pistachio-50">
        <div className="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8">
          <h2 className="text-3xl md:text-4xl font-bold mb-16 text-center text-gray-900">Features</h2>
          <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-8">
            {[
              { icon: Zap, title: 'Fast & Lightweight', description: 'Single Rust binary, instant linting on massive codebases.' },
              { icon: Target, title: 'Configurable Rules', description: 'Server-side exports, component nesting, filename consistency, file organization.' },
              { icon: FileText, title: 'Multiple Formats', description: 'JSON and human-readable output for CI/CD integration and terminal use.' },
              { icon: Cog, title: 'Framework Agnostic', description: 'Works with Next.js, Vite, Astro, React, Vue, and any JavaScript project.' },
              { icon: Cog, title: 'AI-Ready', description: 'Designed to work with coding agents and LLMs for smart scaffolding.' },
              { icon: Zap, title: 'CI/CD Ready', description: 'GitHub Actions, proper exit codes, JSON output for automation.' }
            ].map((feature, i) => {
              const Icon = feature.icon
              return (
                <div key={i} className="bg-white rounded-lg p-6 border border-pistachio-100 hover:border-pistachio-300 hover:shadow-md transition-all">
                  <Icon className="w-8 h-8 text-pistachio-600 mb-3" />
                  <h3 className="text-lg font-semibold text-gray-900 mb-2">{feature.title}</h3>
                  <p className="text-gray-600">{feature.description}</p>
                </div>
              )
            })}
          </div>
        </div>
      </section>

      {/* Documentation Links */}
      <section className="py-16 md:py-24 bg-white">
        <div className="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8">
          <h2 className="text-3xl md:text-4xl font-bold mb-12 text-center text-gray-900">Documentation</h2>
          <div className="grid md:grid-cols-2 gap-6">
            <Link to="/docs/getting-started" className="group block p-6 bg-pistachio-50 border border-pistachio-200 rounded-lg hover:border-pistachio-400 transition-all">
              <h3 className="text-lg font-semibold text-pistachio-700 mb-2 flex items-center gap-2">
                Getting Started <ArrowRight className="w-4 h-4 group-hover:translate-x-1 transition-transform" />
              </h3>
              <p className="text-gray-600">Installation, initialization, and basic usage.</p>
            </Link>
            <Link to="/docs/configuration" className="group block p-6 bg-pistachio-50 border border-pistachio-200 rounded-lg hover:border-pistachio-400 transition-all">
              <h3 className="text-lg font-semibold text-pistachio-700 mb-2 flex items-center gap-2">
                Configuration <ArrowRight className="w-4 h-4 group-hover:translate-x-1 transition-transform" />
              </h3>
              <p className="text-gray-600">Learn how to configure naechste for your project.</p>
            </Link>
            <Link to="/docs/rules" className="group block p-6 bg-pistachio-50 border border-pistachio-200 rounded-lg hover:border-pistachio-400 transition-all">
              <h3 className="text-lg font-semibold text-pistachio-700 mb-2 flex items-center gap-2">
                Rules <ArrowRight className="w-4 h-4 group-hover:translate-x-1 transition-transform" />
              </h3>
              <p className="text-gray-600">Explore all available linting rules with examples.</p>
            </Link>
            <Link to="/docs/schemas" className="group block p-6 bg-pistachio-50 border border-pistachio-200 rounded-lg hover:border-pistachio-400 transition-all">
              <h3 className="text-lg font-semibold text-pistachio-700 mb-2 flex items-center gap-2">
                Schemas <ArrowRight className="w-4 h-4 group-hover:translate-x-1 transition-transform" />
              </h3>
              <p className="text-gray-600">Configuration JSON schemas with IDE support.</p>
            </Link>
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
    </div>
  )
}
