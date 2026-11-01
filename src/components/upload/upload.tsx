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
import { Label } from "@/components/ui/label"
import { Input } from "@/components/ui/input"

interface UploadDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
}

export function UploadDialog({ open, onOpenChange }: UploadDialogProps) {
  const [image, setImage] = useState<File | null>(null)
  const [tags, setTags] = useState("")
  const [title, setTitle] = useState("")
  const [isUploading, setIsUploading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const handleUpload = async () => {
    if (!image || !title.trim()) return

    setIsUploading(true)
    setError(null)

    const formData = new FormData()
    formData.append("file", image)
    formData.append("title", title.trim())

    tags
      .split(",")
      .map((tag) => tag.trim())
      .filter(Boolean)
      .forEach((tag) => formData.append("tags", tag))

    try {
      const response = await fetch("/api/uploads", {
        method: "POST",
        body: formData,
      })

      if (!response.ok) {
        throw new Error(await response.text())
      }

      setImage(null)
      setTitle("")
      setTags("")
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

        <div className="space-y-4">
          <ImageUpload onChange={setImage} />

          <div className="space-y-2">
            <Label>Title *</Label>
            <Input
              id="title"
              placeholder="Enter image title"
              value={title}
              onChange={(e) => setTitle(e.target.value)}
              maxLength={100}
              required
            />
            {image && !title.trim() && (
              <p className="text-xs text-destructive">A title is required.</p>
            )}
          </div>
          <div className="space-y-2">
            <Label>Tags (optional)</Label>
            <Input
              id="tags"
              placeholder="animal, car, snow"
              value={tags}
              onChange={(e) => setTags(e.target.value)}
            ></Input>
            <p className="text-xs text-muted-foreground">
              Separate tags wih commas
            </p>
          </div>
        </div>
        {error && <p className="text-sm text-destructive">{error}</p>}
        <DialogFooter>
          <Button
            variant="outline"
            disabled={isUploading}
            onClick={() => onOpenChange(false)}
          >
            Cancel
          </Button>
          <Button
            disabled={!image || !title.trim() || isUploading}
            onClick={handleUpload}
          >
            {isUploading ? "Uploading..." : "Upload Image"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
