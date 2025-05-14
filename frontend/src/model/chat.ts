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

    constructor(msg:String){
        this.content = msg
    }
}