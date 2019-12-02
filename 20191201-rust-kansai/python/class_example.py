#! /usr/bin/env python
# -*- coding: utf-8 -*-
# vim:fenc=utf-8
#
# Copyright © 2019 fx-kirin <fx.kirin@gmail.com>
#
# Distributed under terms of the MIT license.

"""

"""


class Dog:
    def __init__(self, name, age):
        self.name = name
        self.age = age

    def description(self):
        return "{}, a {} years old dog".format(self.name, self.age)

    def speak_with_description(self, sound):
        return "\"{}\" said {}.".format(sound, self.description())


if __name__ == '__main__':
    dog = Dog("Kevin", 3)
    print(dog.speak_with_description("Bow!"))
