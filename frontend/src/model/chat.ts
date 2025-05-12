export class Chat{
    messages: Message[]
}

export class Message{
    content:String
    isOwn: Boolean

    constructor(msg:String){
        this.content = msg
    }
}