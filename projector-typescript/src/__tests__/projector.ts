import { Operation } from "../config";
import Projector from "../projector";

function getData() {
    return {
        projector: {
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
            }
        }
    }
}


function getProjector(pwd: string, data = getData()): Projector {
    return new Projector({
        args: [],
        operation: Operation.Print,
        pwd,
        config: "hello, frontend",
    }, data)
}

test("getValueAll", function() {
    const proj = getProjector("/foo/bar");
    expect(proj.getValueAll()).toEqual({
        "fem": "is_great",
        "foo": "bar3",
    });
});

test("getValue", function() {
    let proj = getProjector("/foo/bar");
    expect(proj.getValue("foo")).toEqual("bar3");
    proj = getProjector("/foo");
    expect(proj.getValue("foo")).toEqual("bar2");
    expect(proj.getValue("fem")).toEqual("is_great");
});

test("setValue", function() {
    let data = getData();
    let proj = getProjector("/foo/bar",data);
    proj.setValue("foo","bar");
    expect(proj.getValue("foo")).toEqual("bar");

    proj.setValue("fem","is_better");
    expect(proj.getValue("fem")).toEqual("is_better");

    proj = getProjector("/",data);
    expect(proj.getValue("fem")).toEqual("is_great");
});


test("removeValue", function() {
    let proj = getProjector("/foo/bar");
    proj.rmvalue("fem");
    expect(proj.getValue("fem")).toEqual("is_great");
    proj.rmvalue("foo");
    expect(proj.getValue("foo")).toEqual("bar2");
});

