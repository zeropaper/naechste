import { useState } from 'react'
import { Link } from 'react-router'

export function Navigation() {
  const [mobileMenuOpen, setMobileMenuOpen] = useState(false)

  return (
    <nav className="sticky top-0 z-40 bg-white border-b border-pistachio-100">
      <div className="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="flex justify-between items-center py-4">
          <Link to="/" className="flex items-center gap-2 hover:opacity-80 transition">
            <span className="text-2xl">🌿</span>
            <span className="text-xl font-bold text-pistachio-700">naechste</span>
          </Link>
          <div className="hidden md:flex gap-8 items-center">
            <Link to="/docs/getting-started" className="text-gray-600 hover:text-pistachio-600 transition">Getting Started</Link>
            <Link to="/docs/rules" className="text-gray-600 hover:text-pistachio-600 transition">Rules</Link>
            <Link to="/docs/configuration" className="text-gray-600 hover:text-pistachio-600 transition">Configuration</Link>
            <Link to="/docs/schemas" className="text-gray-600 hover:text-pistachio-600 transition">Schemas</Link>
            <a href="https://github.com/zeropaper/naechste" target="_blank" rel="noopener noreferrer" className="px-6 py-3 bg-pistachio-500 text-white font-semibold rounded-lg hover:bg-pistachio-600 transition-colors text-sm">
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
            <Link to="/docs/getting-started" className="block py-2 text-gray-600 hover:text-pistachio-600">Getting Started</Link>
            <Link to="/docs/rules" className="block py-2 text-gray-600 hover:text-pistachio-600">Rules</Link>
            <Link to="/docs/configuration" className="block py-2 text-gray-600 hover:text-pistachio-600">Configuration</Link>
            <Link to="/docs/schemas" className="block py-2 text-gray-600 hover:text-pistachio-600">Schemas</Link>
          </div>
        )}
      </div>
    </nav>
  )
}

export default Navigation
