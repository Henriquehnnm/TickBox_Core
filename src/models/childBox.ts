import type { ActionBox } from './actionBox'
import { Box } from './box'
import type { ParentBox } from './parentBox'

export class ChildBox extends Box {
  parentId: string // Uuid v7
  actionsId?: string[] // Uuid v6
  isAbstract: boolean = true
  duration: number // Ms

  constructor(name: string, description: string, parent: ParentBox, duration: number) {
    super(name, description)
    this.parentId = parent.id
    this.duration = duration
  }

  toggleAbstract() {
    this.isAbstract = !this.isAbstract
  }

  newAction(action: ActionBox) {
    if (this.isAbstract) {
      this.toggleAbstract()
    }

    if (!this.actionsId) {
      this.actionsId = []
    }

    this.actionsId.push(action.id)
  }

  create() {
    return { ...this }
  }
}
