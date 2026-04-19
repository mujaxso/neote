import React from 'react'

export function StatusBar() {
  return (
    <div className="h-6 border-t border-gray-800 flex items-center px-3 text-xs text-gray-400">
      <div className="flex-1">Ready</div>
      <div className="flex items-center space-x-4">
        <span>UTF-8</span>
        <span>Rust</span>
        <span>Ln 1, Col 1</span>
      </div>
    </div>
  )
}
