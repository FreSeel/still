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
    <FileUpload
      value={files}
      onValueChange={handleChange}
      className="w-full"
      accept="image/jpeg,image/png,image/webp"
      maxFiles={1}
      maxSize={10 * 1024 * 1024}
    >
      <FileUploadDropzone className="flex min-h-80 cursor-pointer flex-col items-center justify-center gap-3 rounded-xl border-2 border-dashed border-muted-foreground p-6 text-center">
        <Upload
          className="mb-3 size-12 text-muted-foreground"
          strokeWidth={1.75}
        />

        <p className="text-xl font-medium">
          Drag and drop an image or click here to upload
        </p>
        <p className="text-muted-foreground">
          Accepted formats: .jpeg, .png, .webp
        </p>
        <p className="text-muted-foreground">Max size: 10MB</p>
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
