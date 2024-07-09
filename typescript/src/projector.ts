import { Config } from "./config"
import * as fs from "fs"
import * as path from "path"

export type Data = {
    projector: {
        //pwd
        [key: string]: {
            //key       -> value
            [key: string]: string,
        }
    }
}

const defaultConfig = {
    projector: {}
}

class Projector {

    constructor(private config: Config, private data: Data) {
    }

    getValueAll(): { [key: string]: string } {
        let cur = this.config.pwd;
        let prev = "";
        let out = {};
        const paths: string[] = [];

        do {
            prev = cur;
            paths.push(cur);
            cur = path.dirname(cur);
        } while (cur !== prev);

        paths.reverse().reduce((acc, path) => {
            const value = this.data.projector[path];
            if (value) {
                Object.assign(acc, value);
            }
            return acc;
        }, {});

        return out;
    }

    getValue(key: string): string | undefined {
        let cur = this.config.pwd;
        let prev = "";
        let out: string | undefined;

        do {
            const value = this.data.projector[cur]?.[key];
            if (value) {
                out = value;
                break;
            }
            prev = cur;
            cur = path.dirname(cur);
        } while (cur !== prev);

        return out;
    }

    setValue(key: string, value: string) {
        let dir = this.data.projector[this.config.pwd];

        if (!dir) {
            dir = this.data.projector[this.config.pwd] = {};
        }

        dir[key] = value;
    }

    rmvalue(key: string) {
        const dir = this.data.projector[this.config.pwd];

        if (dir) {
            delete dir[key];
        }
    }

    static fromConfig(config: Config): Projector {
        if (fs.existsSync(config.config)) {
            let data: Data | undefined = undefined;
            try {
                data = JSON.parse(fs.readFileSync(config.config).toString());
            }
            catch (e) {
                data = defaultConfig;
            }
            return new Projector(config, data!);
        }
        return new Projector(config, defaultConfig);
    }
}
