export class Chat{
    id: String
    name: String
    description: String
    messages: Message[]
    isFromPerspectiveA: boolean = true
    isPendingRequest: boolean = false

    constructor(){
        this.messages = [];
    }
}

export class Message{
    id: String
    content:String
    isOwn: Boolean
    avg_rating: Number

    constructor(msg:String){
        this.content = msg
    }
}

export class Comment{
    id: String
    ownerName: String
    content: String
    timestamp: number
}


export class Stat {
	name: string
	value: number
}

export class Passkey {
    name: string
    id: string
    creationDate: Date
}