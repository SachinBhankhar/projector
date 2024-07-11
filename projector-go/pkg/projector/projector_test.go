package projector_test

import (
	"testing"

	"github.com/sachinbhankhar/golearn/pkg/projector"
)

func getData() *projector.Data {
	return &projector.Data{
		Projector: map[string]map[string]string{
			"/": {
				"foo": "bar1",
				"fem": "is_great",
			},
			"/foo": {
				"foo": "bar2",
				"fem": "is_great",
			},
			"/foo/bar": {
				"foo": "bar3",
				"fem": "is_great",
			},
		},
	}
}

func getProjector(pwd string, data *projector.Data) *projector.Projector {
	return projector.CreateProjector(
		&projector.Config{
			Args:      []string{},
			Operation: projector.Print,
			Pwd:       pwd,
			Config:    "Hello, frontend",
		}, data)
}

func test(t *testing.T, proj *projector.Projector, key, value string) {
	v, ok := proj.GetValue(key)

	if !ok {
		t.Errorf("expected to find key %v", key)
	}
	if value != v {
		t.Errorf("expected to find value %v but recieved %v", value, v)
	}
}

func TestGetValue(t *testing.T) {
	data := getData()
	projector := getProjector("/foo/bar", data)
	test(t, projector, "foo", "bar3")
	test(t, projector, "fem", "is_great")
}

func TestSetValue(t *testing.T) {
	data := getData()
	projector := getProjector("/foo/bar", data)
	test(t, projector, "foo", "bar3")
	projector.SetValue("foo", "bar4")
	test(t, projector, "foo", "bar4")
	test(t, projector, "fem", "is_great")
	projector.SetValue("fem", "is_better")
	test(t, projector, "fem", "is_better")
}

func TestRemoveValue(t *testing.T) {
	data := getData()
	projector := getProjector("/foo/bar", data)
	test(t, projector, "foo", "bar3")
	projector.RemoveValue("foo")
	test(t, projector, "foo", "bar2")

	projector = getProjector("/", data)
	test(t, projector, "fem", "is_great")
}
