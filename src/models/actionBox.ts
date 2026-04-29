import { Box } from './box'
import type {ChildBox} from "./childBox.ts";

export class ActionBox extends Box {
  childId: string // Uuid v7
  content?: string // Markdown
  duration: number // Ms

  constructor(
    name: string,
    description: string,
    child: ChildBox,
    duration: number,
    content?: string,
  ) {
    super(name, description)
    this.childId = child.id
    this.content = content
    this.duration = duration
  }

  create(): this {
    return { ...this }
  }
}
