import {ParentBox} from "../models/parentBox.ts";
import {ChildBox} from "../models/childBox.ts";
import {ActionBox} from "../models/actionBox.ts";

import {writeFile} from "node:fs"

export class TickBox {
    parent: ParentBox;
    children: ChildBox[];
    actions: ActionBox[]

    constructor(name: string, description: string | null, start_date: Date, end_date: Date) {
        const safeDescription = description ?? '';
        this.parent = new ParentBox(name, safeDescription, start_date, end_date)
    }

    createChild(name: string, description: string | null, duration: number) {
        const safeDescription = description ?? '';
        const data = new ChildBox(name, safeDescription, this.parent, duration)
        if (!this.children) {
            this.children = [];
        }
        this.children.push(data);
        this.parent.addChild(data)
    }

    createAction(name: string, description: string | null, duration: number, content: string, pid: number) {
        const safeDescription = description ?? '';
        const childPosition = this.parent.childrenId![pid]
        const real = this.children.find(child => child.id === childPosition)
        if (real) {
            const data = new ActionBox(name, safeDescription, real, duration, content)

            if (!this.actions) {
                this.actions = []
            }
            this.actions.push(data)
            this.children[pid].addAction(data)
        }
    }

    // TODO - Criar metodo de validacao

    create() {
        const data = {...this}
        writeFile("./test.json", JSON.stringify(data, null, 2), (err) => {
            console.error(err)
        })
    }
}


// Dev test
const data = new TickBox("Hello World!", null, new Date("2026-04-29"), new Date("2026-05-15"));
data.createChild("Hello World from Child", null, 2000)
data.createChild("Hello World from Child BTW", null, 2000)
data.createAction("Hello World from Action", null, 2000, "# Start new TickBox", 0)
data.createAction("Hello World from Action BTW", null, 2000, "# Start new TickBox", 1)
data.create()
