import { Box } from './box'
import { ChildBox } from './childBox'

export class ParentBox extends Box {
  childrenId?: string[] // Uuid v7
  isDraft: boolean = true
  startDate: Date
  endDate: Date

  constructor(name: string, description: string, startDate: Date, endDate: Date) {
    super(name, description)
    this.startDate = startDate
    this.endDate = endDate
  }

  toggleDraft() {
    this.isDraft = !this.isDraft
  }

  addChild(child: ChildBox) {
    if (this.isDraft) {
      this.toggleDraft()
    }
    if (!this.childrenId) {
      this.childrenId = []
    }
    this.childrenId.push(child.id)
  }

  create(): this {
    return { ...this }
  }
}