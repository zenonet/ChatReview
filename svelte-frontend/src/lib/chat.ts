export class Chat{
    id: String | null = null
    name!: String
    description: String | null = null
    messages: Message[]
    isFromPerspectiveA: boolean = true
    isPendingRequest: boolean = false

    constructor(){
        this.messages = [];
    }
}

export class Message{
    id: String | null = null
    content:String
    isOwn: Boolean = true
    avg_rating: Number = 0

    constructor(msg:String){
        this.content = msg
    }
}

export class Comment{
    id: String | null = null
    ownerName: String | null = null
    content: String = "N/A"
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