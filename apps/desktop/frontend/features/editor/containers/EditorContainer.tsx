import { useState, useEffect } from 'react';
import { CodeEditor } from '@/components/editor/CodeEditor';
import { WorkspaceService } from '@/features/workspace/services/workspaceService';

interface EditorContainerProps {
  filePath?: string;
}

export function EditorContainer({ filePath }: EditorContainerProps) {
  const [content, setContent] = useState<string>('');
  const [language, setLanguage] = useState<string>('rust');
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [fileName, setFileName] = useState<string>('editor.rs');

  useEffect(() => {
    if (filePath) {
      loadFile(filePath);
    } else {
      // Default placeholder content
      setContent(`// Welcome to Zaroxi Editor
// Open a file from the workspace explorer to start editing

// Or create a new file using the command palette (Ctrl+P)`);
      setLanguage('rust');
      setFileName('editor.rs');
    }
  }, [filePath]);

  const loadFile = async (path: string) => {
    setIsLoading(true);
    try {
      const response = await WorkspaceService.openFile({ path });
      setContent(response.content);
      setLanguage(response.language || 'plaintext');
      setFileName(path.split(/[\\/]/).pop() || 'file');
    } catch (error) {
      console.error('Failed to load file:', error);
      setContent(`// Error loading file: ${error instanceof Error ? error.message : 'Unknown error'}`);
      setLanguage('plaintext');
      setFileName('error.txt');
    } finally {
      setIsLoading(false);
    }
  };

  const handleEditorChange = (value: string) => {
    setContent(value);
  };

  const handleEditorSave = async () => {
    if (!filePath) {
      console.warn('No file path to save to');
      return;
    }
    
    try {
      await WorkspaceService.saveFile({
        path: filePath,
        content: content,
      });
      console.log('File saved successfully');
    } catch (error) {
      console.error('Failed to save file:', error);
    }
  };

  return (
    <div className="h-full flex flex-col">
      <div className="border-b border-border px-4 py-2 flex items-center justify-between">
        <div className="text-sm font-medium flex items-center space-x-2">
          <span>{fileName}</span>
          {isLoading && (
            <div className="w-2 h-2 rounded-full bg-blue-500 animate-pulse" />
          )}
        </div>
        <div className="flex items-center space-x-2">
          {filePath && (
            <button
              onClick={handleEditorSave}
              className="px-3 py-1 text-xs bg-primary text-primary-foreground rounded hover:bg-primary/90"
            >
              Save
            </button>
          )}
        </div>
      </div>
      <div className="flex-1 overflow-hidden">
        <CodeEditor
          initialValue={content}
          onChange={handleEditorChange}
          language={language}
          readOnly={false}
        />
      </div>
    </div>
  );
}
