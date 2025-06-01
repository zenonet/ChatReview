

class User{
    username!: string;
    accessToken!: string;
}

function createUserState(){
    let user = $state<User | null>(null);
    return user;
}

export const userState = createUserState();