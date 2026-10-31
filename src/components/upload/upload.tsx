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

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-lg">
        <DialogHeader>
          <DialogTitle>Upload Images</DialogTitle>
          <DialogDescription>Select an image to upload.</DialogDescription>
        </DialogHeader>
        <ImageUpload onChange={setImage} />
        <DialogFooter>
          <Button variant="outline">Cancel</Button>
          <Button
            disabled={!image}
            onClick={() => {
              // ToDo: Call API backend
            }}
          >
            Upload Image
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
