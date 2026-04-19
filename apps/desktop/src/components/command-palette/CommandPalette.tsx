import React, { useState, useEffect } from 'react'

export function CommandPalette() {
  const [isOpen, setIsOpen] = useState(false)
  
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
        e.preventDefault()
        setIsOpen(true)
      }
      if (e.key === 'Escape') {
        setIsOpen(false)
      }
    }
    
    window.addEventListener('keydown', handleKeyDown)
    return () => window.removeEventListener('keydown', handleKeyDown)
  }, [])
  
  if (!isOpen) return null
  
  return (
    <div className="fixed inset-0 bg-black/50 flex items-start justify-center pt-20 z-50">
      <div className="w-[600px] bg-gray-900 rounded-lg border border-gray-700 shadow-2xl">
        <div className="p-4 border-b border-gray-800">
          <input
            type="text"
            placeholder="Type a command or search..."
            className="w-full bg-transparent border-none outline-none text-lg"
            autoFocus
          />
        </div>
        <div className="py-2">
          <div className="px-4 py-2 hover:bg-gray-800 cursor-pointer">
            <div className="font-medium">Open Workspace</div>
            <div className="text-sm text-gray-400">Open a folder or workspace</div>
          </div>
          <div className="px-4 py-2 hover:bg-gray-800 cursor-pointer">
            <div className="font-medium">New File</div>
            <div className="text-sm text-gray-400">Create a new file</div>
          </div>
        </div>
      </div>
    </div>
  )
}
