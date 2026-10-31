import { useState } from "react"
import { Upload } from "lucide-react"
import {
  FileUpload,
  FileUploadDropzone,
  FileUploadItem,
  FileUploadItemDelete,
  FileUploadItemMetadata,
  FileUploadItemPreview,
  FileUploadList,
} from "@/components/ui/file-upload"

interface ImageUploadProps {
  onChange?: (file: File | null) => void
}

export function ImageUpload({ onChange }: ImageUploadProps) {
  const [files, setFiles] = useState<File[]>([])

  const handleChange = (newFiles: File[]) => {
    setFiles(newFiles)
    onChange?.(newFiles[0] ?? null)
  }

  return (
    <FileUpload value={files} onValueChange={handleChange} className="w-full">
      <FileUploadDropzone>
        <Upload />
      </FileUploadDropzone>
      <FileUploadList>
        {files.map((file) => (
          <FileUploadItem value={file}>
            <FileUploadItemPreview />
            <FileUploadItemMetadata />
            <FileUploadItemDelete />
          </FileUploadItem>
        ))}
      </FileUploadList>
    </FileUpload>
  )
}
