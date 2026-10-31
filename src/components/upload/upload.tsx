import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog"

interface UploadDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
}

export function UploadDialog({ open, onOpenChange }: UploadDialogProps) {
  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-lg">
        <DialogHeader>
          <DialogTitle>Upload Images</DialogTitle>
          <DialogDescription>Select an image to upload.</DialogDescription>
        </DialogHeader>
      </DialogContent>
    </Dialog>
  )
}
