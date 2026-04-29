import { Box } from './box'

export class ActionBox extends Box {
  childId: string // Uuid v7
  content?: string
  duration: number // Ms

  constructor(
    name: string,
    description: string,
    childId: string,
    duration: number,
    content?: string,
  ) {
    super(name, description)
    this.childId = childId
    this.content = content
    this.duration = duration
  }

  create() {
    return { ...this }
  }
}
