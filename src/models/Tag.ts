export class Tag {
    id: number = 0;
    name: string = "";

    public constructor(init?:Partial<Tag>) {
        Object.assign(this, init);
    }
    
}