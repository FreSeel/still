import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog"
import { ImageUpload } from "./image-upload"
import { useState } from "react"
import { Button } from "@/components/ui/button"

interface UploadDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
}

export function UploadDialog({ open, onOpenChange }: UploadDialogProps) {
  const [image, setImage] = useState<File | null>(null)
  const [isUploading, setIsUploading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const handleUpload = async () => {
    if (!image) return

    setIsUploading(true)
    setError(null)

    const formData = new FormData()
    formData.append("file", image)

    try {
      const response = await fetch("/api/uploads", {
        method: "POST",
        body: formData,
      })

      if (!response.ok) {
        throw new Error(await response.text())
      }

      setImage(null)
      onOpenChange(false)
    } catch (err) {
      setError(err instanceof Error ? err.message : "Upload failed")
    } finally {
      setIsUploading(false)
    }
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-lg">
        <DialogHeader>
          <DialogTitle>Upload Images</DialogTitle>
          <DialogDescription>Select an image to upload.</DialogDescription>
        </DialogHeader>
        <ImageUpload onChange={setImage} />
        {error && <p className="text-sm text-destructive">{error}</p>}
        <DialogFooter>
          <Button
            variant="outline"
            disabled={isUploading}
            onClick={() => onOpenChange(false)}
          >
            Cancel
          </Button>
          <Button disabled={!image || isUploading} onClick={handleUpload}>
            {isUploading ? "Uploading..." : "Upload Image"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
