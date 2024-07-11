package projector

import (
	"encoding/json"
	"fmt"
	"os"
	"path"
)

type Data struct {
	Projector map[string]map[string]string `json:"projector"`
}

type Projector struct {
	config *Config
	data   *Data
}

func CreateProjector(config *Config, data *Data) *Projector {
	return &Projector{
		config: config,
		data:   data,
	}
}

func (p *Projector) Save() error {
	dir := path.Dir(p.config.Config)

	if _, err := os.Stat(dir); os.IsNotExist(err) {
		err = os.MkdirAll(dir, 0755)
		if err != nil {
			return err
		}
	}

	jsonString, err := json.Marshal(p.data)

	if err != nil {
		return err
	}
	os.WriteFile(p.config.Config, jsonString, 0755)

	return nil
}

func (p *Projector) GetValue(key string) (string, bool) {
	cur := p.config.Pwd
	prev := ""
	out := ""
	found := false

	for cur != prev {
		if dir, ok := p.data.Projector[cur]; ok {
			if value, ok := dir[key]; ok {
				out = value
				found = true
				break
			}
		}

		prev = cur
		cur = path.Dir(cur)
	}

	return out, found
}

func (p *Projector) GetValueAll() map[string]string {
	cur := p.config.Pwd
	out := map[string]string{}
	paths := []string{}
	prev := ""

	for cur != prev {
		paths = append(paths, cur)
		prev = cur
		cur = path.Dir(cur)
	}

	for i := len(paths) - 1; i >= 0; i-- {
		if dir, ok := p.data.Projector[paths[i]]; ok {
			for k, v := range dir {
				out[k] = v
			}
		}
	}

	return out
}

func (p *Projector) SetValue(key, value string) {
	if _, ok := p.data.Projector[p.config.Pwd]; !ok {
		p.data.Projector[p.config.Pwd] = map[string]string{}
	}
	p.data.Projector[p.config.Pwd][key] = value
}

func (p *Projector) RemoveValue(key string) {
	if dir, ok := p.data.Projector[p.config.Pwd]; ok {
		delete(dir, key)
	}
}

func defaultProjector(config *Config) *Projector {
	return &Projector{
		config: config,
		data: &Data{
			Projector: map[string]map[string]string{},
		},
	}
}

func NewProjector(config *Config) *Projector {
	if _, err := os.Stat(config.Config); err == nil {
		contents, err := os.ReadFile(config.Config)
		if err != nil {
			fmt.Printf("%v", err.Error())
			return defaultProjector(config)
		}
		var data Data
		err = json.Unmarshal(contents, &data)

		if err != nil {
			fmt.Printf("%v", err.Error())
			return defaultProjector(config)
		}

		return &Projector{
			data:   &data,
			config: config,
		}
	}
	return defaultProjector(config)
}
