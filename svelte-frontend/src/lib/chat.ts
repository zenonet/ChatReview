export class Chat{
    id: string | null = null
    name!: string
    description: String | null = null
    messages: Message[]
    isFromPerspectiveA: boolean = true
    isPendingRequest: boolean = false

    constructor(){
        this.messages = [];
    }
}

export class Message{
    id: string | null = null
    content:string
    isOwn: Boolean = true
    avg_rating: Number = 0

    constructor(msg:string){
        this.content = msg
    }
}

export class Comment{
    id: string | null = null
    ownerName: string | null = null
    content: string = "N/A"
    timestamp: number = 0
}


export class Stat {
	name!: string
	value!: number
}

export class Passkey {
    name!: string
    id!: string
    creationDate!: Date
}