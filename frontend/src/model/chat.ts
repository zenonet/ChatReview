export class Chat{
    id: String
    name: String
    description: String
    messages: Message[]

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