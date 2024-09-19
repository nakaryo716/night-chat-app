import { CreateRoom } from "../types/room";

const BASE_URL = "http://localhost:3000";

async function apiRequest(endpoint: string, options: RequestInit): Promise<Response> {
    const response = await fetch(`${BASE_URL}${endpoint}`, options);
    return response;
}

export async function createRoomApi(newRoomPayload: CreateRoom): Promise<Response> {
    return apiRequest("/create_room", {
        method: "POST",
        mode: "cors",
        credentials: "include",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify(newRoomPayload),
    });
}

export async function getRoomInfoApi(targetId: string): Promise<Response> {
    return apiRequest(`/room/${targetId}`, {
        method: "GET",
        mode: "cors",
        credentials: "include",
        headers: {
            "Access-Control-Allow-Origin": "http://localhost:3000",
        },
    });
}

export async function getRoomsApi(): Promise<Response> {
    return apiRequest("/room_ls", {
        method: "GET",
        mode: "cors",
        credentials: "include",
        headers: {
            "Access-Control-Allow-Origin": "http://localhost:3000",
        }
    });
}

export async function deleteRoomApi(targetId: string): Promise<Response> {
    return apiRequest(`/delete_room/${targetId}`, {
        method: "DELETE",
        mode: "cors",
        credentials: "include"
    });
}
