const split = (img: HTMLImageElement) => {
    let canvas = document.createElement('canvas');
    const ctx: CanvasRenderingContext2D = canvas.getContext('2d') as CanvasRenderingContext2D
    const n = 3
    let w = img.naturalWidth
    let h = img.naturalHeight
    let clip_w = w / n
    let clip_h = h / n
    canvas.width = clip_w
    canvas.height = clip_h
    const arr: string[] = []
    for (let i = 0; i < n; i++) {
        for (let j = 0; j < n; j++) {
            ctx.drawImage(img, -j * clip_w, -i * clip_h, w, h)
            arr.push(canvas.toDataURL("image/jpeg",0.9))
        }
    }
    return arr
}
const divide = async () => {
    let arr=await preload.then(split)
    return arr
}
const preload = new Promise<HTMLImageElement>((resolve, reject) => {
    let img: HTMLImageElement = new Image();
    img.onload = function () {
        resolve(img)
    }
    img.src = new URL("../assets/image/mouse.jpg",import.meta.url).href;
})

export default divide