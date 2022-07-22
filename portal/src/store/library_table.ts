import {defineStore} from "pinia";
import {delLibraryByName, reqGetLibrarys, reqLibraryTotal} from "@/api";

export const useLibraryStore = defineStore("library", {
    state: () => {
        return {
            librarys: [],
            library_total:0
        }
    },
    actions: {
        async GetLibrarys(limit: number, offset: number) {
            const axiosResponse = await reqGetLibrarys(limit, offset);
            if (axiosResponse.status != 200) {
                return Promise.reject("get librarys failure")
            } else {
                this.librarys = axiosResponse.data
            }
        },
        async DelLibrary(name:string){
            console.log("dellibra",name);
            const axiosResponse = await delLibraryByName(name);
            if (axiosResponse.status!=200){
                return Promise.reject("del library failure");
            }else {
                this.librarys=this.librarys.filter(v=>v.name!=name)
                return true;
            }
        },
        async GetLibraryTotal(){
            console.log("library total")
            const axiosResponse = await reqLibraryTotal();

            if (axiosResponse.status != 200) {
                return Promise.reject("get library total failure")
            }else {
                console.log(axiosResponse.data)
                this.library_total=axiosResponse.data.count;
                return true
            }
        }
    },
    getters:{
        getName:(state)=> {
            return state.librarys.map(v => v.name)
        },
        getCharData:(state)=>{
            return state.librarys.map(v=>{
                return {
                    name: v.name,
                    value: v.size
                }
            })
        },
    },
    }
)