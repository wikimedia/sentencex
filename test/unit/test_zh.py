import pytest

from sentencex import segment

# ruff: noqa: E501
tests = [
    (
        "安永已聯繫周怡安親屬，協助辦理簽證相關事宜，周怡安家屬1月1日晚間搭乘東方航空班機抵達上海，他們步入入境大廳時神情落寞、不發一語。周怡安來自台中，去年剛從元智大學畢業，同年9月加入安永。",
        [
            "安永已聯繫周怡安親屬，協助辦理簽證相關事宜，周怡安家屬1月1日晚間搭乘東方航空班機抵達上海，他們步入入境大廳時神情落寞、不發一語。",
            "周怡安來自台中，去年剛從元智大學畢業，同年9月加入安永。",
        ],
    ),
    ("我们明天一起去看《摔跤吧！爸爸》好吗？好！", ["我们明天一起去看《摔跤吧！爸爸》好吗？", "好！"]),
]


@pytest.mark.parametrize("text,expected_sents", tests)
def test_segment(text, expected_sents):
    assert list(segment("zh", text)) == expected_sents
