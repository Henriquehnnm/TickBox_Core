import { v7 as uuidv7 } from 'uuid'

export abstract class Box {
  id: string
  name: string
  description: string
  archived: boolean
  createdAt: Date
  updatedAt: Date

  constructor(name: string, description: string) {
    const today = new Date()
    this.id = uuidv7()
    this.name = name
    this.description = description
    this.archived = false
    this.createdAt = today
    this.updatedAt = today
  }

  abstract create(): this
}
