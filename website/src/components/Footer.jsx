export function Footer() {
  return (
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
              <li><a href="https://github.com/zeropaper/naechste" target="_blank" rel="noopener noreferrer" className="hover:text-pistachio-400 transition">GitHub</a></li>
              <li><a href="https://github.com/zeropaper/naechste/issues" target="_blank" rel="noopener noreferrer" className="hover:text-pistachio-400 transition">Issues</a></li>
              <li><a href="https://github.com/zeropaper/naechste/releases" target="_blank" rel="noopener noreferrer" className="hover:text-pistachio-400 transition">Releases</a></li>
            </ul>
          </div>
          <div>
            <h4 className="font-semibold text-white mb-4">Community</h4>
            <ul className="space-y-2 text-sm">
              <li><a href="https://github.com/zeropaper/naechste/discussions" target="_blank" rel="noopener noreferrer" className="hover:text-pistachio-400 transition">Discussions</a></li>
              <li><a href="https://github.com/zeropaper/naechste/blob/main/CONTRIBUTING.md" target="_blank" rel="noopener noreferrer" className="hover:text-pistachio-400 transition">Contribute</a></li>
              <li><a href="https://github.com/zeropaper/naechste/blob/main/LICENSE" target="_blank" rel="noopener noreferrer" className="hover:text-pistachio-400 transition">License (MIT)</a></li>
            </ul>
          </div>
        </div>
        <div className="border-t border-gray-800 pt-8 text-center text-sm">
          <p>&copy; 2026 naechste contributors. Built with Rust and ❤️</p>
        </div>
      </div>
    </footer>
  )
}

export default Footer
