

export class User{
    username!: string;
    accessToken!: string;
}

function createUserState(){
    let user = $state<User | null>(null);
    return {
        get user() {
            if (user === null){
                const dat = localStorage.getItem("user");
                if(dat) user = JSON.parse(dat);
            }
            return user
        },
        set: (newUser: User) => {
            user = newUser
            localStorage.setItem("user", JSON.stringify(user));
        },
        reset: () => user = null,
    }
}

export const userState = createUserState();