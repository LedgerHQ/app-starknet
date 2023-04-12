from abc import ABCMeta, abstractmethod
import socket
import requests


class Button(metaclass=ABCMeta):
    @abstractmethod
    def right_click(self):
        ...

    @abstractmethod
    def left_click(self):
        ...

    @abstractmethod
    def both_click(self):
        ...

    @abstractmethod
    def close(self):
        ...


class ButtonFake(Button):
    def right_click(self):
        pass

    def left_click(self):
        pass

    def both_click(self):
        pass

    def close(self):
        pass


class ButtonTCP(Button):
    def __init__(self, url: str, port: int) -> None:
        self.url = url+':'+str(port)

    def right_click(self):
        action = {"action":"press-and-release"}
        requests.post(self.url+'/button/right', json=action)

    def left_click(self):
        action = {"action":"press-and-release"}
        requests.post(self.url+'/button/left', json=action)

    def both_click(self):
        action = {"action":"press-and-release"}
        requests.post(self.url+'/button/both', json=action)

    def close(self):
        pass
