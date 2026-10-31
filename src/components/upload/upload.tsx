import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog"
import { ImageUpload } from "./image-upload"
import { useState } from "react"

interface UploadDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
}

export function UploadDialog({ open, onOpenChange }: UploadDialogProps) {
  const [image, setImage] = useState<File | null>(null)

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-lg">
        <DialogHeader>
          <DialogTitle>Upload Images</DialogTitle>
          <DialogDescription>Select an image to upload.</DialogDescription>
        </DialogHeader>
        <ImageUpload onChange={setImage} />
      </DialogContent>
    </Dialog>
  )
}
