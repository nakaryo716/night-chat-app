import { User } from "../types/user";

const URL = "http://localhost:3000/user_name";

async function apiRequest(options: RequestInit) {
    const response = await fetch (URL, options);    
    return response;

}

export function setUserNameApi(newUserPayload: User): Promise<Response> {
    return apiRequest({
        method: "POST",
        mode: "cors",
        credentials: "include",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify(newUserPayload),
    });
}

export function getUserNameApi(): Promise<Response> {
    return apiRequest({
        method: "GET",
        mode: "cors",
        credentials: "include",
    })
}

export function deleteUserNameApi(): Promise<Response> {
    return apiRequest({
        method: "DELETE",
        mode: "cors",
        credentials: "include",
    });
}
