package cz.cvut.swa.bazaar.productservice.service

import cz.cvut.swa.bazaar.productservice.data.ReviewDTO
import cz.cvut.swa.bazaar.productservice.logger
import org.springframework.beans.factory.annotation.Value
import org.springframework.stereotype.Service
import org.springframework.web.client.RestTemplate

@Service
class ReviewService(private val restTemplate: RestTemplate) {

    @Value("\${review.service.url}")
    private lateinit var url: String

    private val log by logger()

    fun fetchProductReviews(id: String): List<ReviewDTO>? {
        log.debug("> fetchProductReviews - $id")

        val reviewArray = restTemplate.getForObject("$url/reviews/$id", Array<ReviewDTO>::class.java)

        log.debug("< fetchProductReviews")
        return reviewArray?.toList()
    }

}
