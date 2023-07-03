import { json } from '@sveltejs/kit'

export async function GET() {
    const data = await fetch(`http://localhost:3000/terms`)

    console.log(json(data))

    return json(data)
}