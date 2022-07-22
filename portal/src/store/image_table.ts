import {defineStore} from "pinia";
import {
    delImageByName,
    delTagByNameTag,
    reqGetImages,
     reqGetTags,
    reqImageTotal,
    reqTagTotal
} from "@/api";

export const useImageStore = defineStore("image", {
    state: () => {
        return {
            library_choosed:"",
            images: [],
            images_total:0
        }
    },
    actions: {
        async GetImages(library:string,limit: number, offset: number) {
            // this.locale = locale
            const axiosResponse = await reqGetImages(library,limit, offset);
            if (axiosResponse.status != 200) {
                return Promise.reject("get images failure")
            } else {
                this.images = axiosResponse.data
            }
        },
        async DelImage(name:string){
            const axiosResponse = await delImageByName(name);
            if (axiosResponse.status!=200){
                return Promise.reject("delete image failure");
            }else {
                this.images=this.images.filter(v=>v.name!=name)
                return true;
            }
        },
        async GetImageTotal(library:string){
            const axiosResponse = await reqImageTotal(library);
            if (axiosResponse.status != 200) {
                return Promise.reject("get library total failure")
            }else {
                this.images_total=axiosResponse.data.count;
                return true
            }
        },
        async DelTag(name:string,tag:string){
             const axiosResponse = await delTagByNameTag(name,tag);
            if (axiosResponse.status != 200) {
                return Promise.reject("del tag failure")
            }else {
                this.images=this.images.map(v=>{
                    if (v.name==name){
                        v.tags=v.tags.filter(t=>t.tag!=tag)
                    }
                    return v
                })
            }
        },
        // tags_count
        async GetTagTotal(name:string){
            const axiosResponse = await reqTagTotal(name);
            if (axiosResponse.status!=200){
                return Promise.reject("get tag total failure")
            }else {
                this.images=this.images.map(v=>{
                    if (v.name==name){
                        v.tags_count=axiosResponse.data.count
                    }
                    return v
                })
            }
        },
        async UpdateTags(name:string,limit:number,offset:number){
            const axiosResponse = await reqGetTags(name,limit,offset);
            if (axiosResponse.status!=200){
                return Promise.reject("get tag list failure")
            }else {
                this.images=this.images.map(v=>{
                    if (v.name==name){
                        v.tags=axiosResponse.data
                    }
                    return v
                })
            }
        },
    },
    getters:{
        getName:(state)=>{
            return state.images.map(v=>v.name)
        },
        getCharData:(state)=>{
            return state.images.map(v=>{
                return {
                    name:v.name,
                    value:v.size
                }
            })
        }
    }
    }
)