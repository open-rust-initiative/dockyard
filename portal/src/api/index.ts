import request from "./request";

export const reqRegister = (info) => {
    return request({
        url: "/user/register",
        method: "post",
        data: info
    })
}

export const reqLogin = (info) => {
    return request({
        url: "/user/login",
        method: "post",
        data: info
    })
}

export const reqUpdateUser = (info) => {
    return request({
        url: "/user/update_info",
        method: 'put',
        data: info
    })
}

export const reqUpdateUserPasswd = (info) => {
    return request({
        url: "/user/update_passwd",
        method: 'put',
        data: info
    })
}
export const reqtest = () => {
    return request({
        url: "/user/test",
        method: "get"
    })
}
export const reqGetLibrarys=(limit,offset)=>{
    return request({
        url:"/library/list",
        method:"get",
        params:{
            limit:limit,
            offset:offset
        },
    })
}
export const delLibraryByName=(name)=>{
    console.log("delibaaa",name)
    return request({
        url:"/library/remove",
        method:"delete",
        params:{
            name:name
        }
    })
}
export const reqLibraryTotal=()=>{
    return request({
        url:"/library/count",
        method:"get"
    })
}
export const reqGetImages=(library,limit,offset)=>{
    return request({
        url:"/image/list",
        method:"get",
        params:{
            library:library,
            limit:limit,
            offset:offset
        },
    })
}
export const delImageByName=(name)=>{
    // console.log(name)
    return request({
        url:"/image/remove",
        method:"delete",
        params:{
            name:name,
        }
    })
}
export const delTagByNameTag=(name:string,tag:string)=>{
    return request({
        url:"/tag/remove",
        method:"delete",
        params:{
            name:name,
            tag: tag
        }
    })
}
export const reqImageTotal=(library)=>{
    return request({
        url:"/image/count",
        method:"get",
        params:{
            library:library
        }
    })
}
export const reqTagTotal=(name)=>{
    return request({
        url:"/tag/count",
        method:"get",
        params:{
            name:name
        }
    })
}
export const reqGetTags=(name,limit,offset)=>{
    return request({
        url:"/tag/list",
        method:"get",
        params:{
            name:name,
            limit:limit,
            offset:offset
        },
    })
}
